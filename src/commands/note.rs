use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::model::Task;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use surrealdb::opt::PatchOp;

pub(crate) async fn run(
    db: &DB,
    id: String,
    open_editor: bool,
) -> Result<CommandResult<String>, Box<dyn std::error::Error>> {
    let task = db.select_task_by_partial_id(&id).await?;

    let note_path = match task.work_note_path {
        Some(note_path) => {
            if open_editor {
                open_note(&note_path)?;
            }
            note_path
        }
        None => {
            // TODO: break these into separate functions
            let task_id = task.get_id()?;
            let task_name = task.name.as_str();
            let task_description = task.description.as_deref().unwrap_or_default();
            let task_priority = task.priority.as_str();
            let note_path = path_from_id(task_id.as_str());

            // create file
            std::fs::create_dir_all(note_path.parent().unwrap())?;
            let mut f = std::fs::File::create(&note_path)?;

            // write to file
            let task_header = format!("# Task {task_id} - {task_name}\n\n");
            let subheader = format!("## {task_description}, {task_priority}\n\n");
            f.write_all(task_header.as_bytes())?;
            f.write_all(subheader.as_bytes())?;
            f.write_all(b"# Notes\n\n")?;

            // close file
            f.flush()?;

            let note_path_string = note_path.to_string_lossy().to_string();
            // Update task
            let _: Option<Task> = db
                .client
                .upsert(("task", task_id))
                .patch(PatchOp::replace("/work_note_path", Some(&note_path_string)))
                .await?;

            if open_editor {
                open_note(&note_path_string)?;
            }

            note_path_string
        }
    };

    Ok(CommandResult::new(note_path.to_string(), note_path))
}

pub(crate) fn path_from_id(id: &str) -> PathBuf {
    let exe_path = std::env::current_exe().expect("Could not get executable path");
    let dir_path = exe_path
        .parent()
        .expect("Could not get executable directory");
    dir_path.join("tmgr_notes").join(format!("{id}.md"))
}

fn open_note(note_path: &str) -> std::io::Result<ExitStatus> {
    // TODO: Should use $EDITOR environment variable
    Command::new("vi").arg(note_path).spawn()?.wait()
}
