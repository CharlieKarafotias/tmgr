use super::super::{
    db::DB,
    model::{Task, TaskPriority},
};
use std::{collections::BTreeMap, error::Error, iter::FromIterator};

pub(crate) async fn run(
    db: &DB,
    id: String,
    name: Option<String>,
    priority: Option<TaskPriority>,
    description: Option<String>,
) -> Result<String, Box<dyn Error>> {
    if name.is_none() && priority.is_none() && description.is_none() {
        return Err("No fields to update".into());
    }

    let task = db.select_task_by_partial_id(&id).await?;
    let task_id = task.id()?;

    let update_map: BTreeMap<&str, String> = FromIterator::from_iter(
        [
            name.as_ref().map(|name| ("name", name.to_string())),
            priority
                .as_ref()
                .map(|priority| ("priority", priority.into())),
            description
                .as_ref()
                .map(|description| ("description", description.to_string())),
        ]
        .into_iter()
        .flatten(),
    );

    // TODO: follow this model for Complete & Note work path commands as well instead of Patch op
    let _: Option<Task> = db
        .client
        .update(("task", &task_id))
        .merge(update_map)
        .await?;

    Ok(format!("Successfully updated task '{task_id}'"))
}
