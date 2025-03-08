use super::super::{
    db::DB,
    model::{Task, TaskPriority, TmgrError, TmgrErrorKind},
};
use std::fmt;

pub(crate) async fn run(
    db: &DB,
    name: String,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<String, AddError> {
    let mut task_builder = Task::builder()
        .name(&name)
        .priority(priority.unwrap_or_default());
    if let Some(description) = description {
        task_builder = task_builder.description(description);
    }

    let task: Option<Task> = db
        .client
        .create("task")
        .content(task_builder.build())
        .await
        .map_err(|_| AddError {
            kind: AddErrorKind::DatabaseError,
            message: format!("Failed to create task: '{name}'."),
        })?;

    if let Some(task) = task {
        let id = task.id().map_err(|e| AddError {
            kind: AddErrorKind::BadTaskId,
            message: e.to_string(),
        })?;

        Ok(format!("Task '{id}' created successfully"))
    } else {
        Err(AddError {
            kind: AddErrorKind::FailedToCreateTask,
            message: format!("Database did not return a task for '{name}'."),
        })
    }
}

// -- Add Errors ---
#[derive(Debug)]
pub enum AddErrorKind {
    BadTaskId,
    DatabaseError,
    FailedToCreateTask,
}

#[derive(Debug)]
pub struct AddError {
    kind: AddErrorKind,
    message: String,
}

impl fmt::Display for AddError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (add error: {})", self.message, self.kind)
    }
}

impl fmt::Display for AddErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddErrorKind::BadTaskId => write!(f, "Bad task id"),
            AddErrorKind::DatabaseError => write!(f, "Database error"),
            AddErrorKind::FailedToCreateTask => write!(f, "Failed to create task"),
        }
    }
}

impl From<AddError> for TmgrError {
    fn from(err: AddError) -> Self {
        TmgrError::new(TmgrErrorKind::AddCommand, err.to_string())
    }
}
