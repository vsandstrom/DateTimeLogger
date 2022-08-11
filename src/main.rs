#[allow(unused_imports)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use rusqlite::{Connection, Result};
use chrono::prelude::*;
use clap::Parser;
use ws::{listen, Message};


// TODO: 
// [ X ] -> Make sql calls be able to check if user already exists, and if so, use that users ID for
//          new data input.
// [ X ] -> Make a CLI parser for inputting a custom username under which to store associated data
// [ X ] -> Figure out how to use shared ownership of the Connection object from
//          rusqlite to enter new data in the sqlite table at runtime.
//          SQL table
// [  ] -> Migrate to sqlx Rust library.
// [  ] -> Find out if the CLI - Action argument could be used for something, perhaps to run some test case.
// [  ] -> Work on front-end doing api calls to do new DateTime events into sql database

#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long)]
    // #[clap(default_value_t=String::from("Ville Vässla"))]
    /// Name of user 
    name: Option<String>,
    // #[clap(default_value_t=None)]
    #[clap(short, long)]
    /// Action to take with runtime input
    action: Option<String>,
}

fn main() -> Result<()> {

    let args = Cli::parse();

    let name: String = match args.name {
        Some(name) => name,
        // not that clean error handling
        None => "".to_string(), 
    };

    let action: String = match args.action {
        Some(action) => action,
        None => Default::default(), 
    };

    if name.len() != 0 {
        println!("{:?}", &name);
        println!("{:?}", &action);
    }

    let sqlconn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open("db.db")?));

    let ip: String = "127.0.0.1:3012".to_string();

    // ------------------
    // FORMATTED DATETIME
    // ------------------
    

    let dt: DateTime<Local> = Local::now();
    let date: String = dt.format("%Y-%m-%d").to_string();
    let time: String = dt.format("%H:%M:%S").to_string();

    println!("{}\n{}\n", date, time);

    // ------------------------------------------
    // WEBSOCKET COMMUNICATING TO CLIENT-SIDE APP
    // ------------------------------------------
    
    println!("IP: {ip}");

    if name.len() == 0 {

        listen(ip, |out| {

            // Pass a reference to the opened database file into websocket closure.
            let _inconn: Arc<Mutex<Connection>> = Arc::clone(&sqlconn);

            move |msg: Message| {

                let dt: DateTime<Local> = Local::now();
                let date: String = dt.format("%Y-%m-%d").to_string();
                let time: String = dt.format("%H:%M:%S").to_string();

                // should use .try_lock() and handle the Result tuple. quick n dirty...
                let inconn: MutexGuard<Connection> = _inconn.lock().unwrap();

                //--- TRY TO SEE IF USER ALREADY EXISTS IN DB, otherwise build logic to handle inputing
                //--- new user
                // let mut stmt = conn.prepare("select user from users where user = :user").unwrap();
                // let res = stmt.query([msg.to_string()]).unwrap();
                
                // ------------------------------
                // SQL CONNECTION TO SQL DATABASE
                // ------------------------------

                // rusqlite uses execute to run actual sql queries, is it safe?
                inconn.execute(
                    "create table if not exists users (
                        id integer primary key,
                        name text not null
                    )",
                    [],
                ).unwrap();


                inconn.execute(
                    "create table if not exists data (
                        id integer primary key,
                        date text not null,
                        time text not null,
                        user_id integer not null references users(id)
                    )",
                    [],
                ).unwrap();

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
        }).unwrap();

    } else {
        #[allow(unreachable_code)]
        let mut test_data: HashMap<String, Vec<String>> = HashMap::new();

        let conn:Arc<Mutex<Connection>> = Arc::clone(&sqlconn);
        let conn:MutexGuard<Connection> = conn.lock().unwrap();
        
        test_data.insert(name, vec!(date, time));

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