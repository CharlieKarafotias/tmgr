use crate::commands::db::DB;
use crate::commands::model::Task;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Datetime;

pub(crate) async fn run(db: &DB, name: String) -> Result<String, Box<dyn std::error::Error>> {
    let task: Option<Task> = db.client.select(("task", &name)).await?;

    if task.is_some() {
        let _task: Option<Task> = db
            .client
            .update(("task", &name))
            .patch(PatchOp::replace("/completed_at", Datetime::default()))
            .await?;
        Ok(format!("Successfully updated task '{name}' to completed"))
    } else {
        Err(format!("Task with name '{name}' not found").into())
    }
}
