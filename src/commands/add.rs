use super::db::DB;
use super::model::Task;
use crate::cli::model::TaskPriority;
use surrealdb::sql::Datetime;

pub(crate) async fn run(
    db: &DB,
    name: String,
    priority: TaskPriority,
    description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let task: Option<Task> = db
        .client
        .create("task")
        .content(Task {
            id: None,
            name: name.clone(),
            priority: priority.to_string(),
            description,
            work_note: None,
            created_at: Datetime::default(),
            completed_at: None,
        })
        .await
        .map_err(|_| format!("Failed to create task '{name}'."))?;

    if let Some(task) = task {
        let id = task.id.unwrap_or_default();
        Ok(format!("Task '{id}' created successfully"))
    } else {
        Err("Failed to create task".to_string().into())
    }
}
