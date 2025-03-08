use super::super::{
    db::DB,
    model::{Task, TmgrError, TmgrErrorKind},
};
use comfy_table::{ContentArrangement::Dynamic, Table};
use std::fmt;

pub(crate) async fn run(db: &DB, all: bool) -> Result<String, ListError> {
    let tasks: Vec<Task> = if all {
        db.client.select("task").await.map_err(|_| ListError {
            kind: ListErrorKind::DatabaseError,
            message: "Failed to get all tasks".to_string(),
        })?
    } else {
        let query = "SELECT * FROM task WHERE completed_at IS None";
        db.client
            .query(query)
            .await
            .map_err(|_| ListError {
                kind: ListErrorKind::DatabaseError,
                message: "Failed to get in progress tasks".to_string(),
            })?
            .take(0)
            .map_err(|_| ListError {
                kind: ListErrorKind::SerializationError,
                message: "Failed to serialize tasks".to_string(),
            })?
    };

    let mut table = Table::new();
    table.set_content_arrangement(Dynamic).set_header(vec![
        "id",
        "name",
        "priority",
        "description",
        "created_at",
        "completed_at",
    ]);

    // TODO: should have a function implemented for task that allows returning of these values
    // can also filter out specific fields
    tasks.iter().for_each(|t| {
        table.add_row(vec![
            t.id().unwrap_or("ID not found".to_string()).as_str(),
            t.name(),
            t.priority().to_string().as_str(),
            t.description().as_ref().unwrap_or(&"None".to_string()),
            t.created_at().to_string().as_str(),
            t.completed_at()
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("In Progress".to_string())
                .as_str(),
        ]);
    });

    Ok(table.to_string())
}

// --- ListError ---
#[derive(Debug)]
pub enum ListErrorKind {
    DatabaseError,
    SerializationError,
}

#[derive(Debug)]
pub struct ListError {
    kind: ListErrorKind,
    message: String,
}

impl fmt::Display for ListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (list error: {})", self.message, self.kind)
    }
}

impl fmt::Display for ListErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ListErrorKind::DatabaseError => write!(f, "Database error"),
            ListErrorKind::SerializationError => write!(f, "Serialization error"),
        }
    }
}

impl From<ListError> for TmgrError {
    fn from(err: ListError) -> TmgrError {
        TmgrError::new(TmgrErrorKind::ListCommand, err.to_string())
    }
}
