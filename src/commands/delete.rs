use super::super::{db::DB, model::Task};
use std::{error::Error, fs::remove_file, path::Path};

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_note_path = task.work_note_path();
    let task_id = task.id()?;

    // Delete task
    let _: Option<Task> = db.client.delete(("task", &task_id)).await?;

    // Delete note if exists
    if let Some(task_note_path) = task_note_path {
        let note_path = task_note_path;
        // Delete note if exists
        if Path::new(&note_path).exists() {
            remove_file(note_path).expect("Failed to delete note file");
        }
    }

    Ok(format!("Successfully deleted task '{task_id}'"))
}
