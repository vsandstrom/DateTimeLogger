
#[allow(unused_imports)]
use std::collections::HashMap;


#[allow(unused_imports)]
use rusqlite::{Connection, Result};

#[allow(unused_imports)]
use chrono::prelude::*;

#[allow(unused_imports)]
use message_io::network::{self, NetEvent, Transport};

use clap::Parser;

// TODO: 
// [  ] -> Make sql calls be able to check if user already exists, and if so, use that users ID for
//         new data input.
// [  ] -> Work on front-end doing api calls to do new DateTime events into sql database

#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long)]
    #[clap(default_value_t=String::from("Viktor Sandström"))]
    /// Name of user 
    name: String,
    // #[clap(default_value_t=None)]
    #[clap(short, long)]
    /// Action to take with runtime input
    action: Option<String>,
}


fn main() -> Result<()> {

    let args = Cli::parse();
    println!("{:?}", args);

    // ------------------
    // FORMATTED DATETIME
    // ------------------
    
    let dt: DateTime<Local> = Local::now();

    let date: String = dt.format("%Y-%m-%d").to_string();
    let time: String = dt.format("%H:%M:%S").to_string();

    println!("{}\n{}\n", date, time);


    // ------------------------------
    // SQL CONNECTION TO SQL DATABASE
    // ------------------------------


    let conn = Connection::open("medicine.db")?;

    // rusqlite uses execute to run actual sql queries
    conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null
        )",
        [],
    )?;


    conn.execute(
        "create table if not exists data (
            id integer primary key,
            date text not null,
            time text not null,
            user_id integer not null references users(id)
        )",
        [],
    )?;

    let mut test_data = HashMap::new();

    // test_data.insert(String::from("Viktor Sandström"), vec!(date, time));
    test_data.insert(args.name, vec!(date, time));

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

    
    // ------------------------------------------
    // WEBSOCKET COMMUNICATING TO CLIENT-SIDE APP
    // ------------------------------------------

    // TODO

    Ok(())
}
