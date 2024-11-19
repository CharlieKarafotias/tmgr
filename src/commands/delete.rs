use crate::commands::db::DB;
use crate::commands::model::Task;
use std::path::Path;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_note_path = task.work_note_path.clone();
    let task_id = task.get_id()?;

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

    Ok(format!("Successfully deleted task '{task_id}'"))
}
