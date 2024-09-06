use crate::commands::db::DB;
use crate::commands::model::Task;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Datetime;

pub(crate) async fn run(db: &DB, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let _task: Option<Task> = db
        .client
        .update(("task", &name))
        .patch(PatchOp::replace("/completed_at", Datetime::default()))
        .await
        .map_err(|_| format!("Task with name '{name}' not found"))?;

    Ok(format!("Successfully updated task '{name}' to completed"))
}
