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
    let _task: Option<Task> = db
        .client
        .create(("task", &name))
        .content(Task {
            name: name.clone(),
            priority: priority.to_string(),
            description,
            created_at: Datetime::default(),
            completed_at: None,
        })
        .await?;

    Ok(format!("Task '{name}' created successfully"))
}
