use super::super::{db::DB, model::Task};
use std::{
    env::{current_exe, var},
    error::Error,
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use surrealdb::opt::PatchOp;

pub(crate) async fn run(db: &DB, id: String, open_editor: bool) -> Result<String, Box<dyn Error>> {
    let task = db.select_task_by_partial_id(&id).await?;

    if let Some(note_path) = task.work_note_path() {
        if open_editor {
            open_note(note_path)?;
        }
        Ok(note_path.to_string())
    } else {
        let task_id = task.id()?;
        let task_name = task.name();
        let task_description = task.description().as_deref().unwrap_or_default();
        let note_path = path_from_id(task_id.as_str());

        // create file
        create_dir_all(note_path.parent().unwrap())?;
        let mut f = File::create(&note_path)?;

        // write to file
        let task_header = format!("# Task {task_id} - {task_name}\n\n");
        f.write_all(task_header.as_bytes())?;
        if !task_description.is_empty() {
            let subheader = format!("{task_description}\n\n");
            f.write_all(subheader.as_bytes())?;
        }
        f.write_all(b"## Notes\n\n")?;

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

        Ok(note_path_string)
    }
}

pub(crate) fn path_from_id(id: &str) -> PathBuf {
    let exe_path = current_exe().expect("Could not get executable path");
    let dir_path = exe_path
        .parent()
        .expect("Could not get executable directory");
    dir_path.join("tmgr_notes").join(format!("{id}.md"))
}

fn open_note(note_path: &str) -> std::io::Result<ExitStatus> {
    let editor = var("EDITOR").unwrap_or("vi".to_string());
    Command::new(editor).arg(note_path).spawn()?.wait()
}
