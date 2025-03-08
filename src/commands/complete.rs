use super::super::{
    db::DB,
    model::{Task, TmgrError, TmgrErrorKind},
};
use std::fmt;
use surrealdb::{opt::PatchOp, sql::Datetime};

pub(crate) async fn run(db: &DB, id: String) -> Result<String, CompleteError> {
    let task = db
        .select_task_by_partial_id(&id)
        .await
        .map_err(|e| CompleteError {
            kind: CompleteErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    let task_id = task.id().map_err(|e| CompleteError {
        kind: CompleteErrorKind::BadTaskId,
        message: e.to_string(),
    })?;
    let _: Option<Task> = db
        .client
        .upsert(("task", &task_id))
        .patch(PatchOp::replace("/completed_at", Datetime::default()))
        .await
        .map_err(|_| CompleteError {
            kind: CompleteErrorKind::DatabaseError,
            message: "Failed to set task to complete".to_string(),
        })?;

    Ok(format!(
        "Successfully updated task '{task_id}' to completed"
    ))
}

#[derive(Debug)]
pub enum CompleteErrorKind {
    BadTaskId,
    DatabaseError,
}

#[derive(Debug)]
pub struct CompleteError {
    kind: CompleteErrorKind,
    message: String,
}

impl fmt::Display for CompleteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (complete error: {})", self.message, self.kind)
    }
}

impl fmt::Display for CompleteErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompleteErrorKind::BadTaskId => write!(f, "Bad task id"),
            CompleteErrorKind::DatabaseError => write!(f, "Database error"),
        }
    }
}

impl From<CompleteError> for TmgrError {
    fn from(err: CompleteError) -> Self {
        TmgrError::new(TmgrErrorKind::CompleteCommand, err.to_string())
    }
}
