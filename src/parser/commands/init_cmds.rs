//! This file contains the orchestrator functions for initializing the program

use std::fmt;

use crate::parser::{commands::db_cmds, state_mgr::State};

// --- Init Commands ---
pub async fn initialize(state: &mut State) -> Result<(), InitError> {
    let db_name = "init_db".to_string();
    db_cmds::db_set_directory(state, ".".to_string()).map_err(|e| InitError {
        kind: InitErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    db_cmds::db_add(state, db_name.clone())
        .await
        .map_err(|e| InitError {
            kind: InitErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    db_cmds::db_set(state, db_name).map_err(|e| InitError {
        kind: InitErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    println!("Initializer complete!");
    Ok(())
}

// --- Init Errors ---
#[derive(Debug)]
pub struct InitError {
    kind: InitErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum InitErrorKind {
    DatabaseError,
}

impl fmt::Display for InitErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitErrorKind::DatabaseError => write!(f, "database error occurred"),
        }
    }
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (init error: {})", self.message, self.kind)
    }
}
