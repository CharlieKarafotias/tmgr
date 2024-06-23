//! This file contains the orchestrator functions for retrieving the status of the program
use super::super::State;
use super::db_cmds::db::DB;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};
use std::fmt::{self};

pub fn get_status(state: &State) -> Result<(), StatusError> {
    println!("Dumping State Manager Variables...");
    let mut state_mgr_table = Table::new();
    state_mgr_table
        .set_header(vec!["Variable", "Value", "Description"])
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);
    state_mgr_table.add_row(vec![
        "db_dir",
        &state.get_db_dir().unwrap_or("None".to_string()),
        "Current Database Directory",
    ]);
    state_mgr_table.add_row(vec![
        "db_var",
        &state.get_db_var().unwrap_or("None".to_string()),
        "Current Database Name",
    ]);
    println!("{state_mgr_table}");

    println!();
    println!("Dumping Current Database Stats...");
    let db_res = DB::new(state).map_err(|e| StatusError {
        kind: StatusErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    let count = DB::count_tasks(&db_res).map_err(|e| StatusError {
        kind: StatusErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    println!("Record Count: {count}");
    Ok(())
}

// --- Status Errors ---
#[derive(Debug)]
pub struct StatusError {
    kind: StatusErrorKind,
    message: String,
}

#[derive(Debug)]
pub enum StatusErrorKind {
    DatabaseError,
}

impl fmt::Display for StatusErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusErrorKind::DatabaseError => write!(f, "database error occurred"),
        }
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (status error: {})", self.message, self.kind)
    }
}
