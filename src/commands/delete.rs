use crate::commands::db::DB;
use crate::commands::model::Task;
use std::path::Path;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let res: Vec<Task> = db
        .client
        .query(format!(
            "SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\")"
        ))
        .await?
        .take(0)?;

    if res.is_empty() {
        return Err(format!("Task starting with id '{id}' was not found").into());
    }

    if res.len() != 1 {
        return Err("Multiple tasks found, provide more characters of the id"
            .to_string()
            .into());
    }

    let task = &res[0];
    let mut task_id = task.id.clone().unwrap();
    let task_note_path = task.work_note_path.clone();
    task_id = task_id
        .strip_prefix("task:")
        .expect("Task ID should start with task:")
        .to_string();

    // Delete task
    let _: Option<Task> = db.client.delete(("task", &task_id)).await?;

    // Delete note if exists
    if task_note_path.is_some() {
        let note_path = task_note_path.unwrap();
        // Delete note if exists
        if Path::new(&note_path).exists() {
            std::fs::remove_file(note_path).expect("Failed to delete note file");
        }
    }

    Ok(format!("Successfully deleted task starting with id '{id}'"))
}
