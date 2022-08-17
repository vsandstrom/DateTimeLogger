use rusqlite::Connection;

#[allow(dead_code)]
pub fn get_user_entries(conn: &Connection, user: &str) -> Vec<String> {
    let mut stmt = conn.prepare("SELECT * from users WHERE name = ?").unwrap();
    let rows = stmt.query_map([user], |row| row.get(0)).unwrap();   

    let mut names: Vec<String> = Vec::new();
    for row in rows {
        names.push(row.unwrap());
    }
    names
}

pub fn create_db(conn: &Connection) {
    match conn.execute(
        "create table if not exists users (
            id integer primary key,
            name text not null
        )",
        [],
    ) {
        Ok(_) => (), 
        Err(err) => println!("Failed to create 'users'-table. {}", err)
    }

    match conn.execute(
        "create table if not exists data (
            id integer primary key,
            date text not null,
            time text not null,
            user_id integer not null references users(id)
        )",
        [],
    ) {
        Ok(_) => (),
        Err(err) => println!("Failed to create 'data'-table. {}", err)
    };
}

pub fn insert_data(conn: &Connection, user: &str, date: &str, time: &str) {
    match conn.execute(
        "INSERT INTO users (name) values (?1)",
        [user]
    ) {
        Ok(size) => println!("Rows added in 'users'-table: {}", size),
        Err(err) => println!("Failed to update rows: {}", err)
    };

    let last_id: String = conn.last_insert_rowid().to_string();

    match conn.execute(
        "INSERT INTO data (date, time, user_id) values (?1, ?2, ?3)",
        [date, time, &last_id]
    ) {
        Ok(size) => println!("Rows added in 'data'-table: {}", size),
        Err(err) => println!("Failed to update rows: {}", err)
    }
}
