use crate::cli::model::TaskPriority;
use serde::{Deserialize, Deserializer, Serialize};
use surrealdb::sql::{Datetime, Thing};

// defining custom deserializer as surrealdb doesn't support it natively
// see https://github.com/orgs/surrealdb/discussions/2686
fn thing_to_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(Some(t.to_raw()))
}

// TODO: I want these to be only public supers and inside fields private, DO THIS BEFORE MERGE
// TODO: will probably need to move tests into the actual files they are testing instead of separate folder.
// Further, should use tests/ directory for Command tests (high level tests like send command and expect response whereas tests inside files are for low level tests)
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Task {
    #[serde(deserialize_with = "thing_to_string")]
    pub(crate) id: Option<String>,
    pub(crate) name: String,
    pub(crate) priority: String,
    pub(crate) description: Option<String>,
    pub(crate) work_note_path: Option<String>,
    pub(crate) created_at: Datetime,
    pub(crate) completed_at: Option<Datetime>,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Name: \"{}\"", self.name)?;
        writeln!(f, "Priority: \"{}\"", self.priority)?;
        writeln!(
            f,
            "Description: \"{}\"",
            self.description.as_deref().unwrap_or("None")
        )?;
        writeln!(f, "created_at: {}", self.created_at)?;
        writeln!(
            f,
            "completed_at: {}",
            self.completed_at
                .as_ref()
                .map(|d| d.to_string())
                .unwrap_or("In progress".to_string())
        )?;
        Ok(())
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: None,
            name: "a new task".to_string(),
            priority: TaskPriority::High.to_string(),
            description: None,
            work_note_path: None,
            created_at: Datetime::default(),
            completed_at: None,
        }
    }
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
    pub(crate) fn id(&self) -> Result<String, Box<dyn std::error::Error>> {
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn priority(&self) -> &str {
        &self.priority
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn work_note_path(&self) -> &Option<String> {
        &self.work_note_path
    }

    pub fn created_at(&self) -> &Datetime {
        &self.created_at
    }

    pub fn completed_at(&self) -> &Option<Datetime> {
        &self.completed_at
    }
}

#[allow(dead_code)]
impl Task {
    pub fn builder() -> TaskBuilder {
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

#[derive(Default)]
pub struct TaskBuilder {
    id: Option<String>,
    name: Option<String>,
    priority: Option<String>,
    description: Option<String>,
    work_note_path: Option<String>,
    created_at: Option<Datetime>,
    completed_at: Option<Datetime>,
}

impl TaskBuilder {
    #[allow(dead_code)]
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn priority(mut self, priority: impl Into<String>) -> Self {
        self.priority = Some(priority.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    #[allow(dead_code)]
    pub fn work_note_path(mut self, work_note_path: impl Into<String>) -> Self {
        self.work_note_path = Some(work_note_path.into());
        self
    }

    #[allow(dead_code)]
    pub fn created_at(mut self, created_at: Datetime) -> Self {
        self.created_at = Some(created_at);
        self
    }

    #[allow(dead_code)]
    pub fn completed_at(mut self, completed_at: Datetime) -> Self {
        self.completed_at = Some(completed_at);
        self
    }

    pub fn build(self) -> Task {
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
