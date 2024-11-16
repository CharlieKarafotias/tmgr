use crate::commands::db::DB;
use crate::commands::model::Task;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let res: Option<Task> = db
        .client
        .query(format!(
            "SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\") LIMIT 1"
        ))
        .await?
        .take(0)?;
    if let Some(task) = res {
        if let Some(note) = task.work_note_path {
            // TODO: Open note
            Ok(format!(
                "Note file for Task starting with '{id}' opened successfully: {note}"
            ))
        } else {
            // TODO: Create note & open
            Ok(format!(
                "Note file for Task starting with '{id}' created and opened successfully"
            ))
        }
    } else {
        Err(format!("Task starting with id '{id}' was not found").into())
    }
}
