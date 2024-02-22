//! This file contains the orchestrator functions for adding, deleting, listing, and setting operations for the program's database.
pub mod db;
mod db_errors;
mod persistent;
use std::{error::Error, fs, path::PathBuf};

use super::super::state_mgr::State;
use persistent::drop_file_extension;

/// Adds a new database with the specified name.
pub fn db_add(state: &mut State, name: String) {
    match state.get_db_dir() {
        Some(path) => {
            // Create a new database file with provided name.
            let mut db_path = PathBuf::from(&path).join(&name);
            db_path.set_extension("db");

            match db_path.try_exists() {
                Ok(true) => {
                    let e = db_errors::DatabaseError::new(
                        &format!("Unable to add database {}", name),
                        db_errors::DatabaseErrorKind::AlreadyExists,
                    );
                    println!("ERROR: {}", e);
                }
                Ok(false) => match fs::File::create(&db_path) {
                    Ok(_) => println!("Successfully created database with name: {}", name),
                    Err(e) => {
                        println!("ERROR: {}", e);
                    }
                },
                Err(e) => {
                    println!("ERROR: {}", e)
                }
            }
        }
        None => {
            let e = db_errors::DatabaseError::new(
                &format!("Unable to add database {}", name),
                db_errors::DatabaseErrorKind::DirectoryNotSet,
            );
            println!("ERROR: {}", e);
        }
    }
}

/// Deletes the database with the specified name.
pub fn db_delete(state: &mut State, name: String) {
    match state.get_db_dir() {
        Some(path) => {
            let mut db_path = PathBuf::from(&path).join(&name);
            db_path.set_extension("db");
            match db_path.is_file() {
                true => {
                    // remove the file
                    match fs::remove_file(&db_path) {
                        Ok(_) => println!("Successfully deleted database {}", name),
                        Err(e) => println!("ERROR: {}", e),
                    }
                }
                false => {
                    let e = db_errors::DatabaseError::new(
                        &format!("Unable to remove database {}", name),
                        db_errors::DatabaseErrorKind::DoesNotExist,
                    );
                    println!("ERROR: {}", e);
                }
            }
        }
        None => {
            let e = db_errors::DatabaseError::new(
                &format!("Unable to remove database {}", name),
                db_errors::DatabaseErrorKind::DirectoryNotSet,
            );
            println!("ERROR: {}", e);
        }
    }
}

/// Lists all the databases in the current directory and displays their names to the console in alphabetical order.
pub fn db_list(state: &mut State) {
    match state.get_db_dir() {
        Some(path) => match fs::read_dir(path) {
            Ok(dbs) => {
                let mut dbs: Vec<String> = dbs
                    .flatten()
                    .map(|f| drop_file_extension(&f.file_name()).to_string())
                    .collect();
                dbs.sort();
                println!("----Databases-----");
                dbs.iter().for_each(|current| println!("Name: {}", current));
            }
            Err(e) => println!("ERROR: {}", e),
        },
        None => {
            let error = db_errors::DatabaseError::new(
                "Unable to list databases",
                db_errors::DatabaseErrorKind::DirectoryNotSet,
            );
            println!("ERROR: {}", error)
        }
    }
}

/// Sets the current database to the specified name and updates the configuration file's `db_var` variable accordingly.
pub fn db_set(state: &mut State, name: String) {
    match state.get_db_dir() {
        Some(path) => {
            let mut db_path = PathBuf::from(&path).join(&name);
            db_path.set_extension("db");
            if !db_path.is_file() {
                let e: Box<dyn Error> = db_errors::DatabaseError::new(
                    &format!("Unable to set database {}", name),
                    db_errors::DatabaseErrorKind::DoesNotExist,
                )
                .into();
                println!("ERROR: {}", e);
            } else {
                match state.update_var("db_var", &name) {
                    Ok(_) => {
                        println!("Successfully set database to {}", name)
                    }
                    Err(e) => {
                        println!("ERROR: {}", e);
                    }
                };
            }
        }
        None => {
            let e: Box<dyn Error> = db_errors::DatabaseError::new(
                &format!("Unable to set database {}", name),
                db_errors::DatabaseErrorKind::DoesNotExist,
            )
            .into();
            println!("ERROR: {}", e);
        }
    }
}

/// Sets the directory where databases are stored and updates the configuration file's `db_dir` variable accordingly.
pub fn db_set_directory(state: &mut State, path: String) {
    let path_abs = PathBuf::from(&path)
        .canonicalize()
        .expect("ERROR: invalid path");
    let path_abs = path_abs.join("databases");

    // create the directory if it doesn't exist
    if !path_abs.is_dir() {
        match fs::create_dir_all(&path_abs) {
            Ok(_) => {}
            Err(_) => {
                let e: Box<dyn Error> = db_errors::DatabaseError::new(
                    &format!("Unable to set database directory {}", path),
                    db_errors::DatabaseErrorKind::DoesNotExist,
                )
                .into();
                println!("ERROR: {}", e);
                return;
            }
        }
    }

    // update the db_dir variable
    match state.update_var(
        "db_dir",
        path_abs
            .to_str()
            .expect("ERROR: failed to convert path to string"),
    ) {
        Ok(_) => {
            println!(
                "Successfully set database directory to {}",
                path_abs
                    .to_str()
                    .expect("ERROR: failed to convert path to string")
            );
        }
        Err(error) => {
            println!("ERROR: {}", error);
        }
    }
}
