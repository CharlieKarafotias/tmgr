use super::super::{
    db::DB,
    model::{TmgrError, TmgrErrorKind},
};
use comfy_table::{ContentArrangement::Dynamic, Table};
use std::fmt;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, ViewError> {
    let t = db
        .select_task_by_partial_id(&id)
        .await
        .map_err(|e| ViewError {
            kind: ViewErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    let id = t.id().map_err(|e| ViewError {
        kind: ViewErrorKind::BadTaskId,
        message: e.to_string(),
    })?;

    // TODO: refactor with List command (see todo comment in list function)
    let mut table = Table::new();
    table
        .set_content_arrangement(Dynamic)
        .set_header(vec!["Key", "Value"])
        .add_row(vec!["id", &id])
        .add_row(vec!["name", t.name()])
        .add_row(vec!["priority", t.priority().to_string().as_str()])
        .add_row(vec![
            "description",
            t.description().as_ref().unwrap_or(&"None".to_string()),
        ])
        .add_row(vec![
            "work_note_path",
            t.work_note_path().as_ref().unwrap_or(&"None".to_string()),
        ])
        .add_row(vec!["created_at", t.created_at().to_string().as_str()])
        .add_row(vec![
            "completed_at",
            t.completed_at()
                .as_ref()
                .map(|s| s.to_string())
                .unwrap_or("In Progress".to_string())
                .as_str(),
        ]);

    Ok(table.to_string())
}

// -- View Errors ---
#[derive(Debug)]
pub enum ViewErrorKind {
    BadTaskId,
    DatabaseError,
}

#[derive(Debug)]
pub struct ViewError {
    kind: ViewErrorKind,
    message: String,
}

impl fmt::Display for ViewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (view error: {})", self.message, self.kind)
    }
}

impl fmt::Display for ViewErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewErrorKind::BadTaskId => write!(f, "Bad task id"),
            ViewErrorKind::DatabaseError => write!(f, "Database error"),
        }
    }
}

impl From<ViewError> for TmgrError {
    fn from(err: ViewError) -> Self {
        TmgrError::new(TmgrErrorKind::ViewCommand, err.to_string())
    }
}
