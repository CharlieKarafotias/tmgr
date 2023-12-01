use std::path::Path;

use sqlite::{Row, State};

pub fn establish_connection(path: &Path) -> Result<sqlite::Connection, sqlite::Error> {
    // Creates or connects to existing database
    let conn = sqlite::open(path);

    // ensure that the tasks table exists; create if it doesn't
    let query = "
        CREATE TABLE tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
    ";
    conn
}

pub fn table_exists(conn: sqlite::Connection, table_name: &str) -> Result<bool, sqlite::Error> {
    let query = "SELECT COUNT(name) FROM sqlite_master WHERE type='table' AND name = ?;";
    let mut statement = conn.prepare(query)?;
    statement.bind((1, table_name))?;
    // turn to iter and get COUNT(name) column's value. then compare that value using the if statement below
    // TODO: this function is to be used in establish_connection to ensure that table exists before CRUD commands are run
    println!("{}", statement.column_count());
    Ok(false)
    // if count > 0 {
    //     Ok(true)
    // } else {
    //     Ok(false)
    // }
}

pub fn todo_add() {}

pub fn todo_delete() {}

pub fn todo_list() {}

pub fn todo_update() {}
