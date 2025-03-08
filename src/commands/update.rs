use super::super::{
    db::DB,
    model::{Task, TaskPriority, TmgrError, TmgrErrorKind},
};
use std::{collections::BTreeMap, fmt, iter::FromIterator};

pub(crate) async fn run(
    db: &DB,
    id: String,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<String, UpdateError> {
    if name.is_none() && priority.is_none() && description.is_none() {
        return Err(UpdateError {
            kind: UpdateErrorKind::NoFieldsToUpdate,
            message: "No fields to update".to_string(),
        });
    }

    let task = db
        .select_task_by_partial_id(&id)
        .await
        .map_err(|e| UpdateError {
            kind: UpdateErrorKind::DatabaseError,
            message: e.to_string(),
        })?;
    let task_id = task.id().map_err(|e| UpdateError {
        kind: UpdateErrorKind::BadTaskId,
        message: e.to_string(),
    })?;

    let update_map: BTreeMap<&str, String> = FromIterator::from_iter(
        [
            name.as_ref().map(|name| ("name", name.to_string())),
            priority
                .as_ref()
                .map(|priority| ("priority", priority.into())),
            description
                .as_ref()
                .map(|description| ("description", description.to_string())),
        ]
        .into_iter()
        .flatten(),
    );

    // TODO: follow this model for Complete & Note work path commands as well instead of Patch op
    let _: Option<Task> = db
        .client
        .update(("task", &task_id))
        .merge(update_map)
        .await
        .map_err(|_| UpdateError {
            kind: UpdateErrorKind::DatabaseError,
            message: "Failed to update task".to_string(),
        })?;

    Ok(format!("Successfully updated task '{task_id}'"))
}

#[derive(Debug)]
pub enum UpdateErrorKind {
    BadTaskId,
    DatabaseError,
    NoFieldsToUpdate,
}

#[derive(Debug)]
pub struct UpdateError {
    pub kind: UpdateErrorKind,
    pub message: String,
}

impl fmt::Display for UpdateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (update error: {})", self.message, self.kind)
    }
}

impl fmt::Display for UpdateErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UpdateErrorKind::BadTaskId => write!(f, "Bad task id"),
            UpdateErrorKind::DatabaseError => write!(f, "Database error"),
            UpdateErrorKind::NoFieldsToUpdate => write!(f, "No fields to update"),
        }
    }
}

impl From<UpdateError> for TmgrError {
    fn from(err: UpdateError) -> Self {
        TmgrError::new(TmgrErrorKind::UpdateCommand, err.to_string())
    }
}
