use super::super::{
    db::DB,
    model::{CommandResult, Task, TmgrError, TmgrErrorKind},
};
use std::{
    env::{current_exe, var},
    fmt::{self, Formatter},
    fs::{File, create_dir_all},
    io::Write,
    path::PathBuf,
    process::{Command, ExitStatus},
};
use surrealdb::opt::PatchOp;

pub(crate) async fn run(
    db: &DB,
    id: String,
    open_editor: bool,
) -> Result<CommandResult<Task>, NoteError> {
    let task = db
        .select_task_by_partial_id(&id)
        .await
        .map_err(|e| NoteError {
            kind: NoteErrorKind::DatabaseError,
            message: e.to_string(),
        })?;

    if let Some(note_path) = task.work_note_path() {
        if open_editor {
            open_note(note_path)?;
        }
        Ok(CommandResult::new(note_path.to_string(), task))
    } else {
        let task_id = task.id().map_err(|e| NoteError {
            kind: NoteErrorKind::BadTaskId,
            message: e.to_string(),
        })?;
        let task_name = task.name();
        let task_description = task.description().as_deref().unwrap_or_default();
        let note_path = path_from_id(task_id.as_str())?;

        // create file
        let parent_dir = note_path.parent().ok_or(NoteError {
            kind: NoteErrorKind::IOError,
            message: "Could not get parent directory of note".to_string(),
        })?;
        create_dir_all(parent_dir).map_err(|e| NoteError {
            kind: NoteErrorKind::IOError,
            message: e.to_string(),
        })?;
        let mut f = File::create(&note_path).map_err(|e| NoteError {
            kind: NoteErrorKind::IOError,
            message: e.to_string(),
        })?;

        // write to file
        let task_header = format!("# Task {task_id} - {task_name}\n\n");
        f.write_all(task_header.as_bytes()).map_err(|_| NoteError {
            kind: NoteErrorKind::IOError,
            message: "Failed to write to file".to_string(),
        })?;
        if !task_description.is_empty() {
            let subheader = format!("{task_description}\n\n");
            f.write_all(subheader.as_bytes()).map_err(|_| NoteError {
                kind: NoteErrorKind::IOError,
                message: "Failed to write to file".to_string(),
            })?;
        }
        f.write_all(b"## Notes\n\n").map_err(|_| NoteError {
            kind: NoteErrorKind::IOError,
            message: "Failed to write to file".to_string(),
        })?;

        // close file
        f.flush().map_err(|_| NoteError {
            kind: NoteErrorKind::IOError,
            message: "Failed to write to file".to_string(),
        })?;

        let note_path_string = note_path.to_string_lossy().to_string();
        // Update task
        let updated_task: Task = db
            .client
            .upsert(("task", task_id))
            .patch(PatchOp::replace("/work_note_path", Some(&note_path_string)))
            .await
            .map_err(|_| NoteError {
                kind: NoteErrorKind::DatabaseError,
                message: "Failed to update task".to_string(),
            })?
            .ok_or_else(|| NoteError {
                kind: NoteErrorKind::DatabaseError,
                message: "Failed to update task".to_string(),
            })?;

        if open_editor {
            open_note(&note_path_string)?;
        }

        Ok(CommandResult::new(note_path_string, updated_task))
    }
}

pub(super) fn path_from_id(id: &str) -> Result<PathBuf, NoteError> {
    let exe_path = current_exe().map_err(|e| NoteError {
        kind: NoteErrorKind::UnableToDetermineTmgrExecutablePath,
        message: e.to_string(),
    })?;
    let dir_path = exe_path.parent().ok_or(NoteError {
        kind: NoteErrorKind::IOError,
        message: "Could not get parent directory of tmgr executable".to_string(),
    })?;
    Ok(dir_path.join("tmgr_notes").join(format!("{id}.md")))
}

fn open_note(note_path: &str) -> Result<ExitStatus, NoteError> {
    let editor = var("EDITOR").unwrap_or("vi".to_string());
    let res = Command::new(editor)
        .arg(note_path)
        .spawn()
        .map_err(|e| NoteError {
            kind: NoteErrorKind::FailedToOpenEditor,
            message: format!("{e} - HINT: make sure EDITOR is set or vi is installed"),
        })?
        .wait()
        .map_err(|_| NoteError {
            kind: NoteErrorKind::ExecutionError,
            message: "Command was not running as expected".to_string(),
        })?;
    Ok(res)
}

// --- Note Errors ---
#[derive(Debug)]
pub enum NoteErrorKind {
    BadTaskId,
    DatabaseError,
    ExecutionError,
    FailedToOpenEditor,
    IOError,
    UnableToDetermineTmgrExecutablePath,
}

#[derive(Debug)]
pub struct NoteError {
    kind: NoteErrorKind,
    message: String,
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (note error: {})", self.message, self.kind)
    }
}

impl fmt::Display for NoteErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NoteErrorKind::BadTaskId => write!(f, "Bad task id"),
            NoteErrorKind::DatabaseError => write!(f, "Database error"),
            NoteErrorKind::ExecutionError => write!(f, "Execution error"),
            NoteErrorKind::FailedToOpenEditor => write!(f, "Failed to open editor"),
            NoteErrorKind::IOError => write!(f, "IO error"),
            NoteErrorKind::UnableToDetermineTmgrExecutablePath => {
                write!(f, "Unable to determine tmgr executable path")
            }
        }
    }
}

impl From<NoteError> for TmgrError {
    fn from(err: NoteError) -> Self {
        TmgrError::new(TmgrErrorKind::NoteCommand, err.to_string())
    }
}
