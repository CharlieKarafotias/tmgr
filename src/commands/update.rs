use crate::cli::model::TaskPriority;
use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(
    db: &DB,
    current_name: String,
    new_name: Option<String>,
    new_priority: Option<TaskPriority>,
    new_description: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    // If name update, need to create new task as name is used for id
    if let Some(new_name) = new_name {
        let query = format!(
            "BEGIN TRANSACTION;\
             LET $old_task = (SELECT * FROM task WHERE name = \"{current_name}\");\
             IF count($old_task) != 1 {{ THROW \"Task with name '{current_name}' not found\"}};\
             LET $task_obj = $old_task[0];
             CREATE task:`{new_name}` SET name = \"{new_name}\", priority = {}, description = {}, created_at = $task_obj.created_at, completed_at = $task_obj.completed_at;\
             DELETE task:`{current_name}`;\
             COMMIT TRANSACTION;\
            ",
            new_priority.map(|p| format!("\"{}\"", p)).unwrap_or_else(|| "$task_obj.priority".to_string()),
            new_description.map(|d| format!("\"{}\"", d)).unwrap_or_else(|| "$task_obj.description".to_string()),
        );
        let mut db_res = db.client.query(query).await?;
        if !db_res.take_errors().is_empty() {
            return Err(format!("Task with name '{current_name}' not found").into());
        }

        Ok(format!(
            "Successfully updated task '{current_name}' -> '{new_name}'"
        ))
    } else {
        let mut update_content = serde_json::Map::new();
        if let Some(new_priority) = new_priority {
            update_content.insert(
                "priority".to_string(),
                serde_json::Value::String(new_priority.to_string()),
            );
        }
        if let Some(new_description) = new_description {
            update_content.insert(
                "description".to_string(),
                serde_json::Value::String(new_description.clone()),
            );
        }

        if update_content.is_empty() {
            return Err("No fields to update".into());
        }

        let _: Option<Task> = db
            .client
            .update(("task", &current_name))
            .merge(update_content)
            .await
            .map_err(|_| format!("Task with name '{current_name}' not found"))?;

        Ok(format!("Successfully updated task '{current_name}'"))
    }
}
