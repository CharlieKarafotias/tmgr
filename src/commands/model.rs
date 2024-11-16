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
