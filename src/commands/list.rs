use super::super::{
    db::DB,
    model::{CommandResult, TableRow, Task, TmgrError, TmgrErrorKind},
};
use comfy_table::{ContentArrangement::Dynamic, Table};
use std::fmt;

pub(crate) async fn run(db: &DB, all: bool) -> Result<CommandResult<Vec<Task>>, ListError> {
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
    let headers = vec![
        "id".to_string(),
        "name".to_string(),
        "priority".to_string(),
        "description".to_string(),
        "created_at".to_string(),
        "completed_at".to_string(),
    ];
    table.set_content_arrangement(Dynamic).set_header(&headers);

    tasks.iter().for_each(|t| {
        let row = t.to_table_rows_filtered(&headers);
        let vals: Vec<String> = row.iter().map(|(_, v)| v.to_string()).collect();
        table.add_row(vals);
    });

    Ok(CommandResult::new(table.to_string(), tasks))
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
