//! This file contains the orchestrator functions for adding, completing, deleting, listing, and updating a task in a database
use super::super::{State, TaskPriority};
use super::db_cmds::db::DB;

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
                Ok(_) => (), // TODO: complete_todo should return number of rows updated. The commands should handle printing
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
                Ok(_) => (), // TODO: delete_todo should return number of rows deleted. The commands should handle printing
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
            match res {
                Ok(_) => (), // TODO: list_tasks should return vector. The commands should handle printing
                Err(e) => println!("ERROR: {}", e),
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
