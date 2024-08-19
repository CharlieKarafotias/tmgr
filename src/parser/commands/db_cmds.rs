//! This file contains the orchestrator functions for adding, deleting, listing, and setting operations for the program's database.
pub mod db;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use db::DB;
use std::{fmt, fs, path::PathBuf};

use super::super::state_mgr::State;

// --- Database Commands ---
/// Adds a new database with the specified name.
pub async fn db_add(state: &mut State, name: String) -> Result<(), DatabaseError> {
    let db = connect_to_db(state).await?;
    db.set_db(&name).await.map_err(|e| DatabaseError {
        kind: DatabaseErrorKind::SurrealDBError,
        message: e.to_string(),
    })?;

    // Update state variable
    state
        .update_var("db_var", &name)
        .map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::IoError,
            message: e.to_string(),
        })?;
    Ok(())
}

// TODO: if database being deleted is the currently set database, set current db_var in the state to empty
/// Deletes the database with the specified name.
pub async fn db_delete(state: &mut State, name: String) -> Result<(), DatabaseError> {
    let db = connect_to_db(state).await?;
    db.drop_db(&name).await.map_err(|e| DatabaseError {
        kind: DatabaseErrorKind::SurrealDBError,
        message: e.to_string(),
    })
}

/// Lists all the databases in the current directory and displays their names to the console in alphabetical order.
pub async fn db_list(state: &mut State) -> Result<(), DatabaseError> {
    let db = connect_to_db(state).await?;
    let dbs = db.list_dbs().await?;
    let mut table = Table::new();
    table
        .set_header(vec!["Database Name"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    dbs.iter().for_each(|current| {
        table.add_row(vec![current]);
    });

    println!("{table}");
    Ok(())
}

/// Sets the current database to the specified name and updates the configuration file's `db_var` variable accordingly.
pub fn db_set(state: &mut State, name: String) -> Result<(), DatabaseError> {
    let path = state.get_db_dir().ok_or(DatabaseError {
        kind: DatabaseErrorKind::DoesNotExist,
        message: format!("Unable to set database {}", name),
    })?;

    let mut db_path = PathBuf::from(&path).join(&name);
    db_path.set_extension("db");
    if !db_path.is_file() {
        Err(DatabaseError {
            kind: DatabaseErrorKind::DoesNotExist,
            message: format!("Unable to set database {}", name),
        })
    } else {
        state
            .update_var("db_var", &name)
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::IoError,
                message: format!("Unable to set database variable using state manager: {}", e),
            })?;
        println!("Successfully set database to {}", name);
        Ok(())
    }
}

/// Sets the directory where databases are stored and updates the configuration file's `db_dir` variable accordingly.
pub fn db_set_directory(state: &mut State, path: String) -> Result<(), DatabaseError> {
    let path_abs = PathBuf::from(&path)
        .canonicalize()
        .map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::IoError,
            message: format!(
                "Unable to set database directory due to invalid path {}: {}",
                path, e
            ),
        })?;
    let path_abs = path_abs.join("databases");

    // create the directory if it doesn't exist
    if !path_abs.is_dir() {
        fs::create_dir_all(&path_abs).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::DoesNotExist,
            message: format!(
                "Unable to create database directory at {}: {}",
                path_abs.display(),
                e
            ),
        })?;
    }

    // update the db_dir variable
    let db_dir_path = path_abs.to_str().ok_or(DatabaseError {
        kind: DatabaseErrorKind::IoError,
        message: "Unable to convert database directory path to string".to_string(),
    })?;
    state
        .update_var("db_dir", db_dir_path)
        .map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::IoError,
            message: format!("Unable to set database variable using state manager: {}", e),
        })?;

    println!("Successfully set database directory to {}", db_dir_path);
    Ok(())
}

// --- Helper functions ---
/// Removes the file extension from the filename.
fn drop_file_extension(filename: &str) -> String {
    if let Some(dot_index) = filename.rfind('.') {
        filename[..dot_index].to_string()
    } else {
        filename.to_string()
    }
}

// --- Database Errors ---
#[derive(Debug)]
pub struct DatabaseError {
    kind: DatabaseErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum DatabaseErrorKind {
    AlreadyExists,
    DoesNotExist,
    VariableNotSet,
    DirectoryNotSet,
    EntryNotFound,
    IoError,
    SurrealDBError,
}

impl fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseErrorKind::AlreadyExists => write!(
                f,
                "database already exists hint: run `tmgr database set <name>`"
            ),
            DatabaseErrorKind::DoesNotExist => write!(
                f,
                "database does not exists hint: run `tmgr database add <name>`"
            ),
            DatabaseErrorKind::VariableNotSet => write!(
                f,
                "database variable not set, hint: run `tmgr database set <name>`"
            ),
            DatabaseErrorKind::DirectoryNotSet => write!(
                f,
                "database directory not set, hint: run `tmgr database set-directory <dir>`"
            ),
            DatabaseErrorKind::EntryNotFound => write!(f, "entry not found"),
            DatabaseErrorKind::IoError => write!(f, "io error occurred"),
            DatabaseErrorKind::SurrealDBError => write!(f, "SurrealDB error occurred"),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (db error: {})", self.message, self.kind)
    }
}

async fn connect_to_db(state: &mut State) -> Result<DB, DatabaseError> {
    DB::new(state).await
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;

    use crate::parser::state_mgr::{State, States};

    #[tokio::test]
    async fn test_db_add() {
        std::env::set_var("TMGR_TEST", "true");
        let temp_file = NamedTempFile::new().unwrap();
        let mut state = State::new(
            Some(temp_file.path()),
            Some(States {
                db_var: Some("other_db".to_string()),
                db_dir: Some(temp_file.path().to_str().unwrap().to_string()),
            }),
        )
        .unwrap();

        // Add a new database with different name
        super::db_add(&mut state, "test_db".to_string())
            .await
            .unwrap();

        // Expect db_var to be updated to new database name
        assert_eq!(state.get_db_name().unwrap(), "test_db".to_string());
    }
}
