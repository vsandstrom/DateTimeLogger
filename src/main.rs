mod validate;
mod sql_commands;
mod parser;

use crate::validate::{NameOrBool, validate_user};
use crate::parser::{Cli, populate_arg_variables};
#[allow(unused_imports)]
use crate::sql_commands::get_user_entries;

use core::panic;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use rusqlite::{Connection, Result};
use chrono::prelude::*;
use clap::Parser;
use ws::{listen, Message};
#[allow(non_snake_case)]

// TODO: 
// [ X ] -> Make sql calls be able to check if user already exists, and if so, use that users ID for
//          new data input.
// [ X ] -> Make a CLI parser for inputting a custom username under which to store associated data
// [ X ] -> Figure out how to use shared ownership of the Connection object from
//          rusqlite to enter new data in the sqlite table at runtime.
//          SQL tablea
// [ X ] -> Divide struct and functions into submodules
// [  ] -> Migrate to sqlx Rust library.
// [  ] -> Find out if the CLI - Action argument could be used for something, perhaps to run some test case.
// [  ] -> Work on front-end doing api calls to do new DateTime events into sql database
// [  ] -> Try building a chat function around the database and websocket for CLI
// [  ] -> Hashing function to handle multiple users with same name.

fn main() -> Result<()> {
    let args = Cli::parse();

    let cli_name_or_bool: NameOrBool = populate_arg_variables(args.name);

    // Pointer to open database file
    let sqlconn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open("db.db")?));

    // ------------------
    // FORMATTED DATETIME
    // ------------------
    
    // rusqlite uses execute to run actual sql queries, is it safe?
    // Tried to do a simple drop table - sql inject, which did nothing.

    let conn = Arc::clone(&sqlconn);
    let conn = conn.lock().unwrap();

    // Create database if it does not exist
    conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null
        )",
        [],
    ).unwrap();

    conn.execute(
        "create table if not exists data (
            id integer primary key,
            date text not null,
            time text not null,
            user_id integer not null references users(id)
        )",
        [],
    ).unwrap();

    drop(conn);

    // ------------------------------------------
    // WEBSOCKET COMMUNICATING TO CLIENT-SIDE APP
    // ------------------------------------------

    if args.websocket == true && cli_name_or_bool.bool() == false {
        // ip address to websocket listener, and to print to command line.
        let status = "Websocket run:";
        let ip = format!("{}:{}", &args.ip, &args.port);
        println!("{}\n\nIP: {}", status, &ip);

        let listener = listen(ip, |out| {

            // Pass a reference to the opened database file into websocket closure.
            let _inconn: Arc<Mutex<Connection>> = Arc::clone(&sqlconn);

            move |msg: Message| {

                let dt: DateTime<Local> = Local::now();
                let date: String = dt.format("%Y-%m-%d").to_string();
                let time: String = dt.format("%H:%M:%S").to_string();

                // should use .try_lock() and handle the Result tuple. quick n dirty...
                let inconn: MutexGuard<Connection> = _inconn.lock().unwrap();

                // See if user is registered
                let valid = validate_user(&inconn, &msg.to_string());

                // If not in db:
                if !valid {
                    let faulty_input = "Faulty input - User do not exist";
                    println!("{}", faulty_input);
                    out.send(faulty_input)
                } else {

                    // ------------------------------
                    // SQL CONNECTION TO SQL DATABASE
                    // ------------------------------

                    inconn.execute(
                        "INSERT INTO users (name) values (?1)",
                        [msg.to_string()]
                    ).unwrap();

                    let last_id: String = inconn.last_insert_rowid().to_string();

                    inconn.execute(
                        "INSERT INTO data (date, time, user_id) values (?1, ?2, ?3)",
                        [&date, &time, &last_id]
                    ).unwrap();

                    let message: String = format!("{}\n{}, {}", msg.to_string(), &date, &time);

                    println!("{}", &message);
                    out.send(message)
                }
            }
        });

        let _listener = match listener {
            Ok(handle) => handle,
            Err(_) => panic!("Could not connect to IP:port"),
        };

    } else if args.websocket == false {
        let mut test_data: HashMap<String, Vec<String>> = HashMap::new();

        let conn: Arc<Mutex<Connection>> = Arc::clone(&sqlconn);
        
        let status = "Local run:";
        let dt: DateTime<Local> = Local::now();
        let date: String = dt.format("%Y-%m-%d").to_string();
        let time: String = dt.format("%H:%M:%S").to_string();
        let name = String::from(cli_name_or_bool.name());

        let message: String = format!("{}\n\n{}\n{}, {}", status, name, date, time);

        println!("{}", &message);
        // Could be cached for batch insert in .db
        test_data.insert(name, vec!(date, time));

        let conn: MutexGuard<Connection> = conn.lock().unwrap();
        for (users, data) in &test_data {
            conn.execute(
                "INSERT INTO users (name) values (?1)",
                &[&users.to_string()],
            )?;

            let last_id: String = conn.last_insert_rowid().to_string();

            conn.execute(
                "INSERT INTO data (date, time, user_id) values (?1, ?2, ?3)",
                &[&data[0].to_string(), &data[1].to_string(), &last_id],
            )?;
        };
    }
    Ok(())
}