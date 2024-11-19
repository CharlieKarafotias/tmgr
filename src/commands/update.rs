use crate::cli::model::TaskPriority;
use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(
    db: &DB,
    id: String,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    if name.is_none() && priority.is_none() && description.is_none() {
        return Err("No fields to update".into());
    }

    let task = db.select_task_by_partial_id(&id).await?;
    let task_id = task.get_id()?;

    let update_map: std::collections::BTreeMap<&str, String> = std::iter::FromIterator::from_iter(
        [
            name.as_ref().map(|name| ("name", name.clone())),
            priority
                .as_ref()
                .map(|priority| ("priority", priority.to_string())),
            description
                .as_ref()
                .map(|description| ("description", description.clone())),
        ]
        .into_iter()
        .flatten(),
    );

    let _: Option<Task> = db
        .client
        .update(("task", &task_id))
        .merge(update_map)
        .await?;

    Ok(format!("Successfully updated task '{task_id}'"))
}
