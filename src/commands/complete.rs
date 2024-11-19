use crate::commands::db::DB;
use crate::commands::model::Task;
use surrealdb::opt::PatchOp;
use surrealdb::sql::Datetime;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_id = task
        .id
        .clone()
        .expect("Task ID should be set")
        .strip_prefix("task:")
        .expect("Task ID should start with task:")
        .to_string();

    let _: Option<Task> = db
        .client
        .upsert(("task", task_id))
        .patch(PatchOp::replace("/completed_at", Datetime::default()))
        .await?;

    Ok(format!("Successfully updated task '{id}' to completed"))
}
