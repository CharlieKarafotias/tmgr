use surrealdb::sql::Datetime;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub(crate) struct Task {
    pub(crate) name: String,
    pub(crate) priority: String,
    pub(crate) description: Option<String>,
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
