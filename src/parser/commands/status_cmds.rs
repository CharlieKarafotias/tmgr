//! This file contains the orchestrator functions for retrieving the status of the program
use super::super::State;
use super::db_cmds::db::DB;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{ContentArrangement, Table};

pub fn get_status(state: &State) {
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
    let db_res = DB::new(state);
    match db_res {
        Ok(db) => println!(
            "Record Count: {}",
            match DB::count_tasks(&db) {
                Ok(count) => count.to_string(),
                Err(e) => e.to_string(),
            }
        ),
        Err(e) => println!("ERROR: {}", e),
    }
}
