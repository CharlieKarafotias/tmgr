//! This file contains the orchestrator functions for adding, completing, deleting, listing, and updating a task in a database
use super::super::{State, TaskPriority};
use super::db_cmds::db::DB;

pub fn add(state: &mut State, name: String, priority: TaskPriority, description: Option<String>) {
    let db = connect_to_db(state);
    let res = db.add_task(name, priority.to_string(), description);
    println!("{:?}", res);
}
pub fn complete(state: &mut State, id: i64) {
    let db = connect_to_db(state);
    let _ = db.complete_todo(id);
}
pub fn delete(state: &mut State, id: i64) {
    let db = connect_to_db(state);
    let _ = db.delete_todo(id);
}
pub fn list(state: &mut State) {
    let db = connect_to_db(state);
    db.list_tasks().unwrap();
}
pub fn update(
    state: &mut State,
    id: i64,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) {
    let db = connect_to_db(state);
    let mut priority_str: Option<String> = None;
    if let Some(priority) = priority {
        priority_str = Some(priority.to_string());
    }
    let _ = db.update_todo(id, name, priority_str, description);
}

fn connect_to_db(state: &mut State) -> DB {
    DB::new(state).unwrap()
}
