use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let task: Option<Task> = db.client.delete(("task", &name)).await?;

    if task.is_none() {
        return Err(format!("Task with name '{name}' not found").into());
    }
    Ok(format!("Successfully deleted task '{name}'"))
}
