use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::model::Task;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Datetime;

pub(crate) async fn run(
    db: &DB,
    id: String,
) -> Result<CommandResult<Task>, Box<dyn std::error::Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_id = task.get_id()?;

    let res: Option<Task> = db
        .client
        .upsert(("task", &task_id))
        .patch(PatchOp::replace("/completed_at", Datetime::default()))
        .await
        .map_err(|_| format!("Failed to complete task task starting with id: '{id}'."))?;

    if let Some(task) = res {
        Ok(CommandResult::new(
            format!("Successfully updated task '{task_id}' to completed"),
            task,
        ))
    } else {
        Err("Failed to complete task".to_string().into())
    }
}
