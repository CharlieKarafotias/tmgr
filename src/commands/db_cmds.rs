//! This file contains the orchestrator functions for adding, deleting, listing, and setting operations for the program's database.
pub mod db;
mod db_errors;
mod persistent;
use persistent::{change_db, list_dbs, mk_db, rm_db};

/// Adds a new database with the specified name.
///
/// # Arguments
///
/// * `name` - A string representing the name of the new database to be added.
///
/// # Returns
///
/// * `Successfully created database with name: {name}` to console if the database is added successfully.
/// * `The database by the name {name} already exists` to console if a database with the specified name already exists.
///
/// # Errors
/// * Errors if the database already exists.
pub fn db_add(name: String) {
    match mk_db(&name) {
        Ok(_) => {
            println!("Successfully created database with name: {}", name)
        }
        Err(error) => println!("ERROR: {}", error),
    }
}

/// Deletes the database with the specified name.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database to be deleted.
///
///  # Returns
///
/// * `Successfully deleted database` to console if the database is deleted successfully.
/// * `No database exists with the name: {db_name}` to console if no database exists with the specified name.
///
/// # Errors
/// * Errors if database does not exist.
pub fn db_delete(name: String) {
    match rm_db(&name) {
        Ok(_) => {
            println!("Successfully deleted database {}", name)
        }
        Err(error) => println!("ERROR: {}", error),
    }
}

/// Lists all the databases in the current directory and displays their names to the console in alphabetical order.
///
/// # Returns
/// * Lists the names of all the databases in the current directory.
///
/// # Panics
/// * Panics if the database directory cannot be found.
pub fn db_list() {
    match list_dbs() {
        Ok(mut dbs) => {
            println!("----Databases-----");
            dbs.sort();
            dbs.iter().for_each(|current| println!("Name: {}", current));
        }
        Err(error) => println!("ERROR: {}", error),
    }
}

/// Sets the current database to the specified name and updates the .env file accordingly.
///
/// # Arguments
///
/// * `name` - A string representing the name of the database to be set.
///
/// # Examples
///
/// ```
/// db_set("my_database");
/// ```
///
/// # Outputs
///
/// Displays a message indicating whether the database was successfully set or not.
pub fn db_set(name: String) {
    match change_db(&name) {
        Ok(_) => {
            println!("Successfully set database to {}", name)
        }
        Err(error) => println!("ERROR: {}", error),
    }
}
