use std::sync::{Arc, Mutex, MutexGuard};
use ws::{listen, Message};
use rusqlite::Connection;
use chrono::{DateTime, Local};
use crate::sql_commands::insert_data;
use crate::validate::validate_user;

#[allow(dead_code)]
pub fn http_run(){
    todo!()
}

pub fn local_run(user: &str, connection: Arc<Mutex<Connection>>){
    let inner_connection: Arc<Mutex<Connection>> = Arc::clone(&connection);
    
    let status = "Local run:";
    let dt: DateTime<Local> = Local::now();
    let date: String = dt.format("%Y-%m-%d").to_string();
    let time: String = dt.format("%H:%M:%S").to_string();
    let name = String::from(user);

    let message: String = format!("{}\n\n{}\n{}, {}", status, name, date, time);

    println!("{}", &message);
    /*
    Could be cached for batch insert in .db
    let mut test_data: HashMap<String, Vec<String>> = HashMap::new();
    test_data.insert(name, vec!(date, time));
 jj   */

    let inner_connection: MutexGuard<Connection> = inner_connection.lock().unwrap();
    insert_data(&inner_connection, &name, &date, &time);
}

pub fn websocket_run(ip: &str, connection: Arc<Mutex<Connection>>) {
    listen(ip, |out| {

        // Pass a reference to the opened database file into websocket closure.
        let inner_connection: Arc<Mutex<Connection>> = Arc::clone(&connection);

        move |msg: Message| {

            // ------------------
            // FORMATTED DATETIME
            // ------------------

            let dt: DateTime<Local> = Local::now();
            let date: String = dt.format("%Y-%m-%d").to_string();
            let time: String = dt.format("%H:%M:%S").to_string();
            let user: String = msg.to_string();

            let inner_connection: MutexGuard<Connection> = inner_connection.lock().unwrap();

            
            // See if user is registered
            let valid = validate_user(&inner_connection, &user);

            // If not in db:
            if !valid {
                let faulty_input = "Faulty input - User do not exist";
                println!("{}: {}", faulty_input, &user);
                out.send(faulty_input)
            } else {

                // ------------------------------
                // SQL CONNECTION TO SQL DATABASE
                // ------------------------------

                insert_data(&inner_connection, &user, &date, &time);
                let message: String = format!("{}\n{}, {}", &user, &date, &time);

                println!("{}", &message);
                out.send(message)
            }
        }
    }).expect("Unable to open websocket");
}
