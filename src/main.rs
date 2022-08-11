#[allow(unused_imports)]
use std::collections::HashMap;
use std::sync::mpsc::channel;
// use std::time::Duration;

use rusqlite::{Connection, Result};
use chrono::prelude::*;
use std::sync::{Arc, Mutex};
use clap::Parser;
#[allow(unused)]
use ws::{listen, Message};


#[allow(unused_imports)]
// use message_io::node::{self, NodeEvent};
// use message_io::network::{NetEvent, Transport};


// TODO: 
// [ X ] -> Make sql calls be able to check if user already exists, and if so, use that users ID for
//          new data input.
// [ X ] -> Make a CLI parser for inputting a custom username under which to store associated data
// [  ] -> Figure out if there is a need for shared ownership of the Connection object from
//          rusqlite to enter new data in the sqlite table at runtime.
//          SQL table
// [  ] -> Work on front-end doing api calls to do new DateTime events into sql database

#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long)]
    // #[clap(default_value_t=String::from("Viktor Sandström"))]
    /// Name of user 
    name: Option<String>,
    // #[clap(default_value_t=None)]
    #[clap(short, long)]
    /// Action to take with runtime input
    action: Option<String>,
}

// enum Signal {
//     Greet,
// }

fn main() -> Result<()> {

    let args = Cli::parse();

    let name: String = match args.name {
        Some(name) => name,
        None => "".to_string(), // "Viktor Sandström".to_string(),
    };

    let action: String = match args.action {
        Some(action) => action,
        None => Default::default(), 
    };

    if name.len() != 0 {
        println!("{:?}", &name);
        println!("{:?}", &action);

    }


    let sqlconn = Arc::new(Mutex::new(Connection::open("medicine.db")?));

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

    //let conn = Connection::open("medicine.db")?;

    // http::Response();
    
    if name.len() == 0 {

        listen(ip, |out| {
            // use a new thread here to use conn to sql as Arc::new(Mutex::new);

            // Seems unnecesary to use Arc locks to give access to the connection to the sql. 
            // how shold this be handled?
            let _inconn = Arc::clone(&sqlconn);

            // let mut test_data = HashMap::new();
            // test_data.insert(String::from("Viktor Sandström"), vec!(date, time));
            
            move |msg: Message| {
                println!("{}", msg);
                // thread::sleep(Duration::from_secs(2));
                let dt: DateTime<Local> = Local::now();
                let date: String = dt.format("%Y-%m-%d").to_string();
                let time: String = dt.format("%H:%M:%S").to_string();

                // rusqlite uses execute to run actual sql queries
                // let conn = Connection::open("medicine.db").unwrap();

                let inconn = _inconn.lock().unwrap();

                //--- TRY TO SEE IF USER ALREADY EXISTS IN DB, otherwise build logic to handle inputing
                //--- new user
                // let mut stmt = conn.prepare("select user from users where user = :user").unwrap();
                // let res = stmt.query([msg.to_string()]).unwrap();
                

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

                out.send(message)
            }
        }).unwrap();

    } else {
        #[allow(unreachable_code)]
        let mut test_data = HashMap::new();

        // should use .try_lock() and handle the Result tuple. quick n dirty...
        let conn = Arc::clone(&sqlconn);
        let conn = conn.lock().unwrap();
        
        // test_data.insert(String::from("Viktor Sandström"), vec!(date, time));
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

        
    // ------------------------------
    // SQL CONNECTION TO SQL DATABASE
    // ------------------------------


    // cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);

    // cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    // for (color, catnames) in &cat_colors {
    //     conn.execute(
    //         "INSERT INTO cat_colors (name) values (?1)",
    //         &[&color.to_string()],
    //     )?;
    //     let last_id: String = conn.last_insert_rowid().to_string();
    //     for cat in catnames {
    //         conn.execute(
    //             "INSERT INTO cats (name, color_id) values (?1, ?2)",
    //             &[&cat.to_string(), &last_id],
    //         )?;
    //     }
    // }

    Ok(())
}
