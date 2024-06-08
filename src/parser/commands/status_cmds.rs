//! This file contains the orchestrator functions for retrieving the status of the program
use super::super::State;
use super::db_cmds::db::DB;

pub fn get_status(state: &State) {
    println!("Dumping State Manager Variables:");
    println!(
        "db_dir:{} (Current Database Directory)",
        state.get_db_dir().unwrap_or(String::from("None"))
    );
    println!(
        "db_var:{} (Current Database Name)",
        state.get_db_var().unwrap_or(String::from("None"))
    );
    println!("Dumping Current Database Stats:");
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
