use super::super::{
    db::DB,
    model::{CommandResult, Task, TmgrError, TmgrErrorKind},
};
use std::{fmt, fs::remove_file, path::Path};

pub(crate) async fn run(db: &DB, id: String) -> Result<CommandResult<Task>, DeleteError> {
    let task = db
        .select_task_by_partial_id(&id)
        .await
        .map_err(|e| DeleteError {
            kind: DeleteErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    let task_note_path = task.work_note_path();
    let task_id = task.id().map_err(|e| DeleteError {
        kind: DeleteErrorKind::BadTaskId,
        message: e.to_string(),
    })?;

    // Delete task
    let task: Task = db
        .client
        .delete(("task", &task_id))
        .await
        .map_err(|_| DeleteError {
            kind: DeleteErrorKind::FailedToDeleteTask,
            message: "Failed to delete task".to_string(),
        })?
        .ok_or_else(|| DeleteError {
            kind: DeleteErrorKind::FailedToDeleteTask,
            message: "Failed to delete task".to_string(),
        })?;

    // Delete note if exists
    if let Some(task_note_path) = task_note_path {
        let note_path = task_note_path;
        // Delete note if exists
        if Path::new(&note_path).exists() {
            remove_file(note_path).map_err(|e| DeleteError {
                kind: DeleteErrorKind::FailedToDeleteNote,
                message: e.to_string(),
            })?;
        }
    }

    Ok(CommandResult::new(
        format!("Successfully deleted task '{task_id}'"),
        task,
    ))
}

// -- Delete Errors ---
#[derive(Debug)]
pub enum DeleteErrorKind {
    BadTaskId,
    DatabaseError,
    FailedToDeleteTask,
    FailedToDeleteNote,
}

#[derive(Debug)]
pub struct DeleteError {
    pub kind: DeleteErrorKind,
    pub message: String,
}

impl fmt::Display for DeleteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (delete error: {})", self.message, self.kind)
    }
}

impl fmt::Display for DeleteErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeleteErrorKind::BadTaskId => write!(f, "Bad task id"),
            DeleteErrorKind::DatabaseError => write!(f, "Database error"),
            DeleteErrorKind::FailedToDeleteNote => write!(f, "Failed to delete note"),
            DeleteErrorKind::FailedToDeleteTask => write!(f, "Failed to delete task"),
        }
    }
}

impl From<DeleteError> for TmgrError {
    fn from(err: DeleteError) -> Self {
        TmgrError::new(TmgrErrorKind::DeleteCommand, err.to_string())
    }
}
