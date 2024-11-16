use crate::commands::db::DB;
use crate::commands::model::Task;
use std::io::Write;
use std::path::PathBuf;
use surrealdb::opt::PatchOp;

pub(crate) async fn run(db: &DB, id: String) -> Result<String, Box<dyn std::error::Error>> {
    let res: Option<Task> = db
        .client
        .query(format!(
            "SELECT * from task WHERE string::starts_with(<string> id, \"task:{id}\") LIMIT 1"
        ))
        .await?
        .take(0)?;

    if let Some(task) = res {
        if let Some(note_path) = task.work_note_path {
            // TODO: Open note in editor
            Ok(note_path)
        } else {
            let task_id = task
                .id
                .as_deref()
                .unwrap_or_default()
                .strip_prefix("task:")
                .unwrap();
            let task_name = task.name.as_str();
            let task_description = task.description.as_deref().unwrap_or_default();
            let task_priority = task.priority.as_str();
            let note_path = path_from_id(task_id);

            // create file
            std::fs::create_dir_all(note_path.parent().unwrap())?;
            let mut f = std::fs::File::create(&note_path)?;

            // write to file
            let task_header = format!("# Task {task_id} - {task_name}\n");
            let subheader = format!("## {task_description}, {task_priority}\n");
            f.write_all(task_header.as_bytes())?;
            f.write_all(subheader.as_bytes())?;
            f.write_all(b"# Notes\n")?;

            // close file
            f.flush()?;

            // Update task
            let _: Option<Task> = db
                .client
                .upsert(("task", task_id))
                .patch(PatchOp::replace(
                    "/work_note_path",
                    Some(note_path.to_string_lossy().to_string()),
                ))
                .await?;

            // TODO: Open note in editor
            Ok(note_path.to_string_lossy().to_string())
        }
    } else {
        Err(format!("Task starting with id '{id}' was not found").into())
    }
}

fn path_from_id(id: &str) -> PathBuf {
    let exe_path = std::env::current_exe().expect("Could not get executable path");
    let dir_path = exe_path
        .parent()
        .expect("Could not get executable directory");
    dir_path.join("tmgr_notes").join(format!("{id}.md"))
}
