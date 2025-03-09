use super::super::{
    db::DB,
    model::{CommandResult, TmgrError, TmgrErrorKind},
};
use std::{env::current_exe, fmt};

pub(crate) async fn run(db: &DB) -> Result<CommandResult<Status>, StatusError> {
    let status = Status {
        tmgr_executable_path: current_exe()
            .map(|p| p.display().to_string())
            .map_err(|_| StatusError {
                kind: StatusErrorKind::UnableToDetermineTmgrExecutablePath,
                message: "Unable to determine executable location".to_string(),
            })?,
        db_file_path: DB::get_db_file_path()
            .map_err(|e| StatusError {
                kind: StatusErrorKind::UnableToDetermineDbFilePath,
                message: e.to_string(),
            })?
            .display()
            .to_string(),
        task_status: get_number_of_tasks(db).await?,
    };

    Ok(CommandResult::new(status.to_string(), status))
}

pub(crate) struct Status {
    tmgr_executable_path: String,
    db_file_path: String,
    task_status: TaskCount,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "File locations:")?;
        writeln!(f, "  tmgr executable: {}", self.tmgr_executable_path)?;
        writeln!(f, "  database: {}", self.db_file_path)?;
        writeln!(f, "General statistics:")?;
        writeln!(f, "  completed tasks: {}", self.task_status.completed)?;
        writeln!(f, "  in progress tasks: {}", self.task_status.in_progress)?;
        writeln!(f, "  total tasks: {}", self.task_status.total)
    }
}

pub(crate) struct TaskCount {
    completed: i32,
    in_progress: i32,
    total: i32,
}

async fn get_number_of_tasks(db: &DB) -> Result<TaskCount, StatusError> {
    let mut db_res = db
        .client
        .query("SELECT count() as total, count(completed_at != None) as completed  FROM task GROUP BY total;")
        .await
        .map_err(|_| StatusError {
            kind: StatusErrorKind::DatabaseError,
            message: "Unable to determine number of tasks in current database".to_string()
        })?;

    let total: Option<i32> = db_res.take("total").map_err(|_| StatusError {
        kind: StatusErrorKind::SerializationError,
        message: "Token 'total' not found in database response".to_string(),
    })?;
    let completed: Option<i32> = db_res.take("completed").map_err(|_| StatusError {
        kind: StatusErrorKind::SerializationError,
        message: "Token 'completed' not found in database response".to_string(),
    })?;

    let mut task_count = TaskCount {
        completed: 0,
        in_progress: 0,
        total: 0,
    };
    if let Some(total) = total {
        task_count.total = total;
    }
    if let Some(completed) = completed {
        task_count.completed = completed;
    }

    task_count.in_progress = task_count.total - task_count.completed;
    Ok(task_count)
}

#[derive(Debug)]
pub enum StatusErrorKind {
    DatabaseError,
    SerializationError,
    UnableToDetermineTmgrExecutablePath,
    UnableToDetermineDbFilePath,
}

#[derive(Debug)]
pub struct StatusError {
    kind: StatusErrorKind,
    message: String,
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (status error: {})", self.message, self.kind)
    }
}

impl fmt::Display for StatusErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StatusErrorKind::DatabaseError => write!(f, "Database error"),
            StatusErrorKind::SerializationError => write!(f, "Serialization error"),
            StatusErrorKind::UnableToDetermineTmgrExecutablePath => {
                write!(f, "Unable to determine tmgr executable path")
            }
            StatusErrorKind::UnableToDetermineDbFilePath => {
                write!(f, "Unable to determine database file path")
            }
        }
    }
}

impl From<StatusError> for TmgrError {
    fn from(err: StatusError) -> Self {
        TmgrError::new(TmgrErrorKind::StatusCommand, err.to_string())
    }
}
