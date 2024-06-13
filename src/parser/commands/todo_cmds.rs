//! This file contains the orchestrator functions for adding, completing, deleting, listing, and updating a task in a database
use super::super::{State, TaskPriority};
use super::db_cmds::db::DB;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub fn add(state: &mut State, name: String, priority: TaskPriority, description: Option<String>) {
    let db = connect_to_db(state);
    match db {
        Ok(db) => {
            let res = db.add_task(name, priority.to_string(), description);
            match res {
                Ok(_) => println!("Successfully added task"),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

pub fn complete(state: &mut State, id: i64) {
    let db = connect_to_db(state);
    match db {
        Ok(db) => {
            let res = db.complete_todo(id);
            match res {
                Ok(_) => println!("Successfully set task {id} as completed"),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

pub fn delete(state: &mut State, id: i64) {
    let db = connect_to_db(state);
    match db {
        Ok(db) => {
            let res = db.delete_todo(id);
            match res {
                Ok(_) => println!("Successfully deleted task {id}"),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

pub fn list(state: &mut State) {
    let db = connect_to_db(state);
    match db {
        Ok(db) => {
            let res = db.list_tasks();
            if let Ok(tasks) = res {
                let mut table = Table::new();
                table
                    .set_header(vec![
                        "ID",
                        "Name",
                        "Priority",
                        "Description",
                        "Created On",
                        "Completed On",
                    ])
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic);

                for task in tasks {
                    table.add_row(vec![
                        task.id.to_string(),
                        task.name,
                        task.priority,
                        task.description.unwrap_or_else(|| "NULL".to_string()),
                        task.created_on,
                        task.completed_on
                            .unwrap_or_else(|| "IN-PROGRESS".to_string()),
                    ]);
                }
                println!("{table}");
            } else {
                println!("ERROR: unable to list tasks");
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}
pub fn update(
    state: &mut State,
    id: i64,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) {
    let db = connect_to_db(state);
    match db {
        Ok(db) => {
            let priority_str: Option<String> = priority.map(|p| p.to_string());

            let res = db.update_todo(id, name, priority_str, description);
            match res {
                Ok(_) => println!("Successfully updated task"),
                Err(e) => println!("ERROR: {}", e),
            }
        }
        Err(e) => println!("ERROR: {}", e),
    }
}

fn connect_to_db(state: &mut State) -> Result<DB, Box<dyn std::error::Error>> {
    DB::new(state)
}
