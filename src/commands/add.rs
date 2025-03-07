use super::super::cli::model::TaskPriority;
use super::{db::DB, model::Task};
use std::error::Error;

pub(crate) async fn run(
    db: &DB,
    name: String,
    priority: TaskPriority,
    description: Option<String>,
) -> Result<String, Box<dyn Error>> {
    let mut task_builder = Task::builder().name(&name).priority(priority);
    if let Some(description) = description {
        task_builder = task_builder.description(description);
    }

    let task: Option<Task> = db
        .client
        .create("task")
        .content(task_builder.build())
        .await
        .map_err(|_| format!("Failed to create task '{name}'."))?;

    if let Some(task) = task {
        Ok(format!("Task '{}' created successfully", task.id()?))
    } else {
        Err("Failed to create task".to_string().into())
    }
}
