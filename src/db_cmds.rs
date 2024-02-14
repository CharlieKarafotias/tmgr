//! This file contains the orchestrator functions for adding, deleting, listing, and setting operations for the program's database.
use crate::persistent::{change_db, list_dbs, mk_db, rm_db};

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
/// # Panics
/// * Panics if file creation fails.
pub fn db_add(name: String) {
    if let Ok(file_created) = mk_db(&name) {
        if file_created {
            println!("Successfully created database with name: {}", name);
        } else {
            println!("The database by the name {} already exists", name);
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
/// # Panics
///
/// Panics if the file to be removed is not found.
pub fn db_delete(name: String) {
    if let Ok(file_removed) = rm_db(&name) {
        if file_removed {
            println!("Successfully deleted database");
        } else {
            println!("No database exists with the name: {}", name);
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
    println!("----Databases-----");
    let mut dbs = list_dbs();
    dbs.sort();
    dbs.iter().for_each(|curr| println!("Name: {}", curr));
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
    if let Ok(is_updated) = change_db(&name) {
        if is_updated {
            println!("Successfully set database to {}", name);
        } else {
            println!("No database exists with the name: {}", name);
        }
    };
}
