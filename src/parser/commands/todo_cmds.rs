//! This file contains the orchestrator functions for adding, completing, deleting, listing, and updating a task in a database
use std::fmt;

use super::super::{State, TaskPriority};
use super::db_cmds::db::DB;
use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, ContentArrangement, Table};

// --- Todo Commands ---
pub async fn add(
    state: &mut State,
    name: String,
    priority: TaskPriority,
    description: Option<String>,
) -> Result<(), TodoError> {
    let db = connect_to_db(state).await?;

    db.add_task(name, priority.to_string(), description)
        .await
        .map_err(|e| TodoError {
            kind: TodoErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    println!("Successfully added task");
    Ok(())
}

pub async fn complete(state: &mut State, id: String) -> Result<(), TodoError> {
    let db = connect_to_db(state).await?;
    db.complete_todo(id.clone()).await.map_err(|e| TodoError {
        kind: TodoErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    println!("Successfully set task {id} as completed");
    Ok(())
}

pub async fn delete(state: &mut State, id: String) -> Result<(), TodoError> {
    let db = connect_to_db(state).await?;
    db.delete_todo(id.clone()).await.map_err(|e| TodoError {
        kind: TodoErrorKind::DatabaseError,
        message: e.to_string(),
    })?;
    println!("Successfully deleted task {id}");
    Ok(())
}

pub async fn list(state: &mut State, should_show_all: bool) -> Result<(), TodoError> {
    let db = connect_to_db(state).await?;
    let tasks = db.list_tasks().await.map_err(|e| TodoError {
        kind: TodoErrorKind::DatabaseError,
        message: e.to_string(),
    })?;

    // TODO: reimplement
    // let mut table = Table::new();
    // table
    //     .set_header(vec![
    //         "ID",
    //         "Name",
    //         "Priority",
    //         "Description",
    //         "Created On",
    //         "Completed On",
    //     ])
    //     .load_preset(UTF8_FULL)
    //     .apply_modifier(UTF8_ROUND_CORNERS)
    //     .set_content_arrangement(ContentArrangement::Dynamic);

    // for task in tasks {
    //     // if should_show_all is false, only show tasks that are not completed
    //     if !should_show_all {
    //         if task.completed_on.is_none() {
    //             table.add_row(vec![
    //                 task.id.to_string(),
    //                 task.name,
    //                 task.priority,
    //                 task.description.unwrap_or_else(|| "NULL".to_string()),
    //                 task.created_on,
    //                 task.completed_on
    //                     .unwrap_or_else(|| "IN-PROGRESS".to_string()),
    //             ]);
    //         }
    //     } else {
    //         table.add_row(vec![
    //             task.id.to_string(),
    //             task.name,
    //             task.priority,
    //             task.description.unwrap_or_else(|| "NULL".to_string()),
    //             task.created_on,
    //             task.completed_on
    //                 .unwrap_or_else(|| "IN-PROGRESS".to_string()),
    //         ]);
    //     }
    // }
    // println!("{table}");
    println!("{tasks:#?}");
    Ok(())
}

pub async fn update(
    state: &mut State,
    id: String,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<(), TodoError> {
    let db = connect_to_db(state).await?;

    let priority_str: Option<String> = priority.map(|p| p.to_string());

    db.update_todo(id, name, priority_str, description)
        .await
        .map_err(|e| TodoError {
            kind: TodoErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    println!("Successfully updated task");
    Ok(())
}

// --- Helper Functions ---
async fn connect_to_db(state: &mut State) -> Result<DB, TodoError> {
    DB::new(state).await.map_err(|e| TodoError {
        kind: TodoErrorKind::UnableToConnectToDatabase,
        message: e.to_string(),
    })
}

// --- Todo Errors ---
#[derive(Debug)]
pub struct TodoError {
    kind: TodoErrorKind,
    message: String,
}

#[derive(Debug)]
enum TodoErrorKind {
    DatabaseError,
    UnableToConnectToDatabase,
}

impl fmt::Display for TodoErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TodoErrorKind::DatabaseError => write!(f, "database error"),
            TodoErrorKind::UnableToConnectToDatabase => write!(f, "unable to connect to database"),
        }
    }
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (todo error: {})", self.message, self.kind)
    }
}
