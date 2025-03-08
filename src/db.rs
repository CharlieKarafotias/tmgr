use super::model::{Task, TmgrError, TmgrErrorKind};
use std::{
    fmt::{self, Formatter},
    path::PathBuf,
};
use surrealdb::{
    Surreal,
    engine::any::{Any, connect},
};

// TODO: should client be private?
pub(super) struct DB {
    pub(super) client: Surreal<Any>,
}

impl DB {
    pub(super) async fn new() -> Result<Self, DBError> {
        let client = connect(format!(
            "surrealkv://{}",
            Self::get_db_file_path()?.display()
        ))
        .await
        .map_err(|_| DBError {
            kind: DBErrorKind::IOError,
            message: "Could not create/connect to file database".to_string(),
        })?;
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .map_err(|_| DBError {
                kind: DBErrorKind::DatabaseError,
                message: "Failed to set namespace and database".to_string(),
            })?;
        Ok(Self { client })
    }

    pub(super) async fn new_test() -> Result<Self, DBError> {
        let client = connect("mem://").await.map_err(|_| DBError {
            kind: DBErrorKind::IOError,
            message: "Could not connect to memory database".to_string(),
        })?;
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .map_err(|_| DBError {
                kind: DBErrorKind::DatabaseError,
                message: "Failed to set namespace and database".to_string(),
            })?;
        Ok(Self { client })
    }

    pub(super) fn get_db_file_path() -> Result<PathBuf, DBError> {
        let exe_path = std::env::current_exe().map_err(|e| DBError {
            kind: DBErrorKind::UnableToDetermineTmgrExecutablePath,
            message: e.to_string(),
        })?;
        let dir_path = exe_path.parent().ok_or(DBError {
            kind: DBErrorKind::IOError,
            message: "Could not get parent directory of tmgr executable".to_string(),
        })?;
        Ok(dir_path.join("tmgr_db"))
    }

    /// Select a task from the database by a partial id.
    ///
    /// Returns a Task if exactly one task is found with the given id.
    /// Returns an error if no tasks are found, or if multiple tasks are found.
    ///
    /// The id should be a prefix of the full id of the task you want to select.
    /// The full id of each task is "task:<id>", where <id> is the id you
    /// provided when you added the task.
    pub(super) async fn select_task_by_partial_id(
        &self,
        id: impl Into<String>,
    ) -> Result<Task, DBError> {
        let id_string = id.into();
        let query = format!(
            "SELECT * from task WHERE string::starts_with(<string> id, \"task:{}\")",
            &id_string
        );

        let res: Vec<Task> = self
            .client
            .query(query)
            .await
            .map_err(|_| DBError {
                kind: DBErrorKind::DatabaseError,
                message: "Failed to get tasks".to_string(),
            })?
            .take(0)
            .map_err(|_| DBError {
                kind: DBErrorKind::SerializationError,
                message: "Failed to deserialize tasks".to_string(),
            })?;

        if res.is_empty() {
            return Err(DBError {
                kind: DBErrorKind::NoTasksFound,
                message: format!("Task starting with id '{}' was not found", &id_string),
            });
        }

        if res.len() != 1 {
            return Err(DBError {
                kind: DBErrorKind::MultipleTasksFound,
                message: "Multiple tasks found, provide more characters of the id".to_string(),
            });
        }

        let task = res.into_iter().next().ok_or(DBError {
            kind: DBErrorKind::ExpectedOneTask,
            message: "Expected one task".to_string(),
        })?;
        Ok(task)
    }
}

#[derive(Debug)]
pub enum DBErrorKind {
    DatabaseError,
    ExpectedOneTask,
    IOError,
    MultipleTasksFound,
    NoTasksFound,
    SerializationError,
    UnableToDetermineTmgrExecutablePath,
}

#[derive(Debug)]
pub struct DBError {
    kind: DBErrorKind,
    message: String,
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (db error: {})", self.message, self.kind)
    }
}

impl fmt::Display for DBErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DBErrorKind::DatabaseError => write!(f, "Database error"),
            DBErrorKind::ExpectedOneTask => write!(f, "Expected one task"),
            DBErrorKind::IOError => write!(f, "IO error"),
            DBErrorKind::MultipleTasksFound => write!(f, "Multiple tasks found"),
            DBErrorKind::NoTasksFound => write!(f, "No tasks found"),
            DBErrorKind::SerializationError => write!(f, "Serialization error"),
            DBErrorKind::UnableToDetermineTmgrExecutablePath => {
                write!(f, "Unable to determine tmgr executable path")
            }
        }
    }
}

impl From<DBError> for TmgrError {
    fn from(err: DBError) -> Self {
        TmgrError::new(TmgrErrorKind::Tmgr, err.to_string())
    }
}
