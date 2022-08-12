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
//          SQL table
// [  ] -> Migrate to sqlx Rust library.
// [  ] -> Find out if the CLI - Action argument could be used for something, perhaps to run some test case.
// [  ] -> Work on front-end doing api calls to do new DateTime events into sql database
// [  ] -> Try building a chat function around the database and websocket for CLI


#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long)]
    // #[clap(default_value_t=String::from("Default Defaultsson"))]
    /// Name of user 
    name: Option<String>,
    #[clap(short, long)]
    #[clap(default_value_t=String::from("127.0.0.1"))]
    /// IP adress for websocket to attach to
    ip: String,
    #[clap(short, long)]
    #[clap(default_value_t=String::from("6666"))]
    /// Port for websocket to listen to
    port: String,
    #[clap(short, long, takes_value=false)]
    #[clap()]
    /// Determines local or remote data input
    websocket: bool,
}

fn populate_arg_variables(arg: Option<String>) -> String {
    let a = match arg {
        Some(arg) => arg,
        None => panic!("Unvalid arguments as <name>")
    };

    return a;
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let name = populate_arg_variables(args.name);

    // Pointer to open database file
    let sqlconn: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open("db.db")?));

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

    if args.websocket == true {
        // ip address to websocket listener, and to print to command line.
        let ip = format!("{}:{}", &args.ip, &args.port);
        println!("IP: {}", &ip);

        let listener = listen(ip, |out| {

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
                // Tried to do a simple drop table - sql inject, which did nothing.
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
        });

        let _listener = match listener {
            Ok(handle) => handle,
            Err(_) => panic!("Could not connect to IP:port"),
        };

    } else if args.websocket == false {
        let mut test_data: HashMap<String, Vec<String>> = HashMap::new();

        let conn: Arc<Mutex<Connection>> = Arc::clone(&sqlconn);
        let conn: MutexGuard<Connection> = conn.lock().unwrap();
        
        // Could be cached for batch insert in .db
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