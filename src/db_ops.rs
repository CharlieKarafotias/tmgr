use std::fs;
use std::path::Path;

pub fn establish_connection(path: &Path) -> Result<sqlite::Connection, sqlite::Error> {
    // Creates or connects to existing database
    let conn = sqlite::open(path)?;

    // ensure that the tasks table exists; create if it doesn't
    let query = "
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        )
    ";
    conn.execute(query)?;
    Ok(conn)
}

pub fn remove_db(path: &Path) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}

pub fn list_dbs() {}

pub fn todo_add() {}

pub fn todo_delete() {}

pub fn todo_list() {}

pub fn todo_update() {}
