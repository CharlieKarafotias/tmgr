use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let task: Option<Task> = db.client.select(("task", &name)).await?;
    if let Some(task) = task {
        Ok(task.to_string())
    } else {
        Err(format!("Task with name '{name}' not found").into())
    }
}
