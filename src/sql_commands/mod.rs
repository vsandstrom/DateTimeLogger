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

