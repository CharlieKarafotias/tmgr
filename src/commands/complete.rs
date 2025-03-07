use super::{db::DB, model::Task};
use std::error::Error;
use surrealdb::{opt::PatchOp, sql::Datetime};

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_id = task.id()?;
    let _: Option<Task> = db
        .client
        .upsert(("task", &task_id))
        .patch(PatchOp::replace("/completed_at", Datetime::default()))
        .await?;

    Ok(format!(
        "Successfully updated task '{task_id}' to completed"
    ))
}
