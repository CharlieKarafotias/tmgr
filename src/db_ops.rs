use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn form_db_path(name: &str) -> PathBuf {
    // check if database folder exists; create one if not
    let db_dir_exists = Path::new("./databases").exists();
    if !db_dir_exists {
        fs::create_dir("./databases").expect("Failed to create database directory");
    }
    let path_str = format!("./databases/{name}.db");
    PathBuf::from_str(&path_str).expect("Error in form_db_path")
}

pub fn establish_connection(name: &str) -> Result<sqlite::Connection, sqlite::Error> {
    // Creates or connects to existing database
    let path = form_db_path(name);
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

pub fn remove_db(name: &str) -> std::io::Result<()> {
    let path = form_db_path(name);
    fs::remove_file(path)?;
    Ok(())
}

pub fn list_dbs() {
    let dir_entries = fs::read_dir("./databases").expect("Error reading databases directory");
    dir_entries.for_each(|x| {
        let filename = x.unwrap().file_name();
        // Removes '.db' from ends
        let only_db_name = filename.to_str().unwrap().strip_suffix(".db").unwrap();
        println!("{}", only_db_name)
    })
}

pub fn set_active_db(name: &str) {
    // ensure db exists
    let db_exists = Path::new(format!("./databases/{name}.db").as_str()).is_file();
    // TODO: finish this func
    // ensure .env file exists
    // update .env file to correct db
}

pub fn todo_add() {}

pub fn todo_delete() {}

pub fn todo_list() {}

pub fn todo_update() {}
