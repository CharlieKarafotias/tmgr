//! This file contains the orchestrator functions for adding, deleting, listing, and setting operations for the program's database.
pub mod db;
mod db_errors;
mod persistent;
use super::super::state_mgr::State;
use persistent::{change_db, env_check_state, list_dbs, mk_db, rm_db};

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
    if db_state_ok() {
        match mk_db(&name) {
            Ok(_) => {
                println!("Successfully created database with name: {}", name)
            }
            Err(error) => println!("ERROR: {}", error),
        }
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
    if db_state_ok() {
        match rm_db(&name) {
            Ok(_) => {
                println!("Successfully deleted database {}", name)
            }
            Err(error) => println!("ERROR: {}", error),
        }
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
    if db_state_ok() {
        match list_dbs() {
            Ok(mut dbs) => {
                println!("----Databases-----");
                dbs.sort();
                dbs.iter().for_each(|current| println!("Name: {}", current));
            }
            Err(error) => println!("ERROR: {}", error),
        }
    }
}

/// Sets the current database to the specified name and updates the .env file's `db_var` variable accordingly.
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

/// Sets the directory where databases are stored and updates the configuration file's `db_dir` variable accordingly.
pub fn db_set_directory(mut state: State, path: String) {
    match state.update_var("db_dir", &path) {
        Ok(_) => {
            println!("Successfully set database directory to {}", path)
        }
        Err(error) => println!("ERROR: {}", error),
    }
}

/// Function to ensure the env state is set correctly
///
/// Returns
/// * bool true if env state is set correctly
/// * bool false if env state is not set
///
/// Errors
/// * Errors if env state is not set; indicating which fields aren't correct
fn db_state_ok() -> bool {
    if let Err(error) = env_check_state() {
        println!("ERROR: {}", error);
        return false;
    }
    true
}
