
#![allow(non_snake_case)]

mod validate;
mod sql_commands;
mod parser;
mod actions;

use crate::parser::{Cli, populate_arg_variables};
use crate::validate::NameOrBool;
use crate::sql_commands::create_db;
#[allow(unused_imports)]
use crate::actions::{websocket_run, local_run, http_run};

use std::sync::{Arc, Mutex};
use rusqlite::{Connection, Result};
use clap::Parser;

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
    // Parse Argv into Cli struct
    let args = Cli::parse();
    // Hacky solution for handling when there is no username from CLI during websocket run.
    let cli_name_or_bool: NameOrBool = populate_arg_variables(args.name);

    // Pointer to open database file
    let sqlconnection: Arc<Mutex<Connection>> = Arc::new(Mutex::new(Connection::open("db.db")?));
    let connection = Arc::clone(&sqlconnection);
    let connection = connection.lock().unwrap();

    // Create database if it does not exist
    create_db(&connection);
    drop(connection);

    // ------------------------------------------
    // WEBSOCKET COMMUNICATING TO CLIENT-SIDE APP
    // ------------------------------------------

    // if websocket is true and no username has been given at command line:
    if args.websocket == true && !cli_name_or_bool.bool() {
        // ip address to websocket listener, and to print to command line.
        let status = "Websocket run:";
        let ip = format!("{}:{}", &args.ip, &args.port);
        println!("{}\n\nIP: {}", status, &ip);

        websocket_run(&ip, Arc::clone(&sqlconnection));

    // else if websocket is off and there is a username from commandline:
    } else if args.websocket == false && cli_name_or_bool.bool(){

        local_run(cli_name_or_bool.name(), Arc::clone(&sqlconnection));
        
    }
    Ok(())
}
