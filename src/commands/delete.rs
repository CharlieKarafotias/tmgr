use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::model::Task;
use std::path::Path;

pub(crate) async fn run(
    db: &DB,
    id: String,
) -> Result<CommandResult<Task>, Box<dyn std::error::Error>> {
    let task = db.select_task_by_partial_id(&id).await?;
    let task_note_path = task.work_note_path.clone();
    let task_id = task.get_id()?;

    // Delete task
    let res: Option<Task> = db.client.delete(("task", &task_id)).await?;

    // Delete note if exists
    if task_note_path.is_some() {
        let note_path = task_note_path.unwrap();
        // Delete note if exists
        if Path::new(&note_path).exists() {
            std::fs::remove_file(note_path).expect("Failed to delete note file");
        }
    }

    if let Some(res) = res {
        Ok(CommandResult::new(
            format!("Successfully deleted task '{task_id}'"),
            res,
        ))
    } else {
        Err("Failed to delete task".to_string().into())
    }
}
