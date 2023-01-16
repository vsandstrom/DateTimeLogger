#![allow(non_snake_case)]

use rusqlite::Connection;
use regex::Regex;


pub struct NameOrBool {
    name: String,
    bool: bool
}

impl NameOrBool {
    pub fn new(name: String, bool: bool) -> NameOrBool {
        NameOrBool { name: (name), bool: (bool) }
    }

    pub fn bool(&self) -> bool {
        return self.bool;
    }
    
    pub fn name(&self) -> &str {
        return &self.name;
    }
}

pub fn validate_user(conn: &Connection, user: &str) -> bool {
    // Need user id hash to id which user if multiple with same name exist.
    // Use Regex to identify if user has a probable name.
    let re: Regex = Regex::new(r"[a-öA-Ö]\s[a-öA-Ö]").unwrap();
    // let emptyStr: Regex = Regex::new(r"[\s]").unwrap();
    if !re.is_match(user) && user == String::from(' ') {
        println!("Not a valid input");
        return false;
    } 
    // else if emptyStr.is_match(user) {
    //     println!("No string sent");
    //     return false;
    // }

    // if user already exist in db, then it is valid to enter more data
    let mut stmt = conn.prepare("SELECT * from users WHERE name = ?").unwrap();
    // let rows = &stmt.query(rusqlite::params![user]);
    // let _rows = match rows {
    //     Ok(_) => true,
    //     Err(_) => false,
    // };
    let rows = stmt.query_map([user], |row| row.get(1)).unwrap();   

    let mut names: Vec<String> = Vec::new();
    for row in rows {
        names.push(row.unwrap());
    }
    if names.len() > 0{
        return true;
    } 
    false
}