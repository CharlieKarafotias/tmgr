use clap::ValueEnum;
use serde::{Deserialize, Deserializer, Serialize};
use std::{error::Error, fmt::Display};
use surrealdb::sql::{Datetime, Thing};

// -- Task --
/// Represents a task.
///
/// This is the structure of the data in the database, and must match the
/// structure of the data in the database.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
pub(super) struct Task {
    #[serde(deserialize_with = "thing_to_string")]
    id: Option<String>,
    name: String,
    priority: TaskPriority,
    description: Option<String>,
    work_note_path: Option<String>,
    created_at: Datetime,
    completed_at: Option<Datetime>,
}

impl Task {
    /// Returns the ID of the task as a string, without the "task:" prefix.
    ///
    /// If the task ID is not set, this function returns an error.
    ///
    /// If the task ID is set, but does not start with "task:", this function returns an error.
    /// The expected format of task IDs is "task:<id>", where <id> is the id you
    /// provided when you added the task.
    ///
    pub(super) fn id(&self) -> Result<String, Box<dyn Error>> {
        let actual_id = &self.id;
        if let Some(actual_id) = actual_id {
            let id = actual_id.strip_prefix("task:");
            match id {
                Some(id) => Ok(id.to_string()),
                None => Err(
                    format!(
                        "Task ID from database is not prefixed with 'task:'. Expected 'task:<id>', but got '{actual_id}'"
                    ).into()
                )
            }
        } else {
            Err("Task ID is not set".into())
        }
    }

    /// The name of the task.
    ///
    /// This is the short description of the task you provided when you added the task.
    pub(super) fn name(&self) -> &str {
        &self.name
    }

    /// The priority of the task.
    ///
    /// This is the priority of the task you provided when you added the task.
    pub(super) fn priority(&self) -> &TaskPriority {
        &self.priority
    }

    /// An optional long description of the task.
    ///
    /// This is the description of the task you provided when you added the task.
    /// This is optional, as you did not have to provide a description when you added the task.
    pub(super) fn description(&self) -> &Option<String> {
        &self.description
    }

    /// The path to a markdown file to store notes associated with the task.
    ///
    /// This is the path you provided when you ran the `note` command.
    /// This is optional, as you did not have to provide a path when you ran the `note` command.
    pub(super) fn work_note_path(&self) -> &Option<String> {
        &self.work_note_path
    }

    /// The time at which the task was added.
    ///
    /// This is the time at which the task was added to the database.
    pub(super) fn created_at(&self) -> &Datetime {
        &self.created_at
    }

    /// The time at which the task was completed.
    ///
    /// This is the time at which the task was marked as completed.
    /// This is optional, as the task may not have been completed yet.
    pub(super) fn completed_at(&self) -> &Option<Datetime> {
        &self.completed_at
    }

    /// Constructs a new `TaskBuilder` with all fields set to `None`.
    ///
    /// This is a convenient way to start building a `Task` incrementally by
    /// setting only the desired fields using the builder pattern.
    pub(super) fn builder() -> TaskBuilder {
        TaskBuilder {
            id: None,
            name: None,
            priority: None,
            description: None,
            work_note_path: None,
            created_at: None,
            completed_at: None,
        }
    }
}

// defining custom deserializer as surrealdb doesn't support it natively
// see https://github.com/orgs/surrealdb/discussions/2686
fn thing_to_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(Some(t.to_raw()))
}
// -- Task --

// -- TaskPriority --
/// Represents the priority of a task.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ValueEnum)]
pub(super) enum TaskPriority {
    #[default]
    Low,
    Medium,
    High,
}

impl From<&TaskPriority> for String {
    fn from(priority: &TaskPriority) -> Self {
        match priority {
            TaskPriority::Low => "Low".to_string(),
            TaskPriority::Medium => "Medium".to_string(),
            TaskPriority::High => "High".to_string(),
        }
    }
}

impl Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <&TaskPriority as Into<String>>::into(self).fmt(f)
    }
}
// -- TaskPriority --

// -- TaskBuilder --
#[derive(Default)]
pub(super) struct TaskBuilder {
    id: Option<String>,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
    work_note_path: Option<String>,
    created_at: Option<Datetime>,
    completed_at: Option<Datetime>,
}

impl TaskBuilder {
    /// Sets the ID of the task to the given value.
    ///
    /// This is optional, and defaults to `None`.
    #[allow(dead_code)]
    pub(super) fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the name of the task to the given value.
    ///
    /// This is required, and defaults to `String::default()`.
    pub(super) fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the priority of the task to the given value.
    ///
    /// This is required, and defaults to `TaskPriority::Low`.
    pub(super) fn priority(mut self, priority: TaskPriority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Sets the description of the task to the given value.
    ///
    /// This is optional, and defaults to `None`.
    pub(super) fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the path to the markdown file to store notes associated with the task.
    ///
    /// This is optional, and defaults to `None`.
    #[allow(dead_code)]
    pub(super) fn work_note_path(mut self, work_note_path: impl Into<String>) -> Self {
        self.work_note_path = Some(work_note_path.into());
        self
    }

    /// Sets the time at which the task was added to the given value.
    ///
    /// This is optional, and defaults to `Datetime::default()`.
    #[allow(dead_code)]
    pub(super) fn created_at(mut self, created_at: Datetime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    /// Sets the time at which the task was completed to the given value.
    ///
    /// This is optional, and defaults to `None`.
    #[allow(dead_code)]
    pub(super) fn completed_at(mut self, completed_at: Datetime) -> Self {
        self.completed_at = Some(completed_at);
        self
    }

    /// Builds a `Task` from the current state of the builder.
    pub(super) fn build(self) -> Task {
        Task {
            id: self.id,
            name: self.name.unwrap_or_default(),
            priority: self.priority.unwrap_or_default(),
            description: self.description,
            work_note_path: self.work_note_path,
            created_at: self.created_at.unwrap_or_default(),
            completed_at: self.completed_at,
        }
    }
}

// -- TaskBuilder --
