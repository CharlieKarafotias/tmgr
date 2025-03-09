use super::super::{
    db::DB,
    model::{TableRow, TmgrError, TmgrErrorKind},
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

    let _ = t.id().map_err(|e| ViewError {
        kind: ViewErrorKind::BadTaskId,
        message: e.to_string(),
    })?;

    let mut table = Table::new();
    table.set_content_arrangement(Dynamic);
    table.set_header(vec!["Key", "Value"]);
    t.to_table_rows().iter().for_each(|(k, v)| {
        table.add_row(vec![k, v]);
    });

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
