use crate::commands::model::Task;
use std::path::PathBuf;
use surrealdb::engine::any::connect;
use surrealdb::{engine::any::Any, Surreal};

pub(crate) struct DB {
    pub(crate) client: Surreal<Any>,
}

impl DB {
    pub(crate) async fn new() -> Self {
        let client = connect(format!(
            "surrealkv://{}",
            Self::get_db_file_path().display()
        ))
        .await
        .expect("Could not create/connect to file database");
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .expect("Could not set namespace and database");
        Self { client }
    }

    pub(crate) async fn new_test() -> Self {
        let client = connect("mem://")
            .await
            .expect("Could not connect to memory database");
        client
            .use_ns("tmgr_ns")
            .use_db("tmgr_db")
            .await
            .expect("Could not set namespace and database");
        Self { client }
    }

    pub(crate) fn get_db_file_path() -> PathBuf {
        let exe_path = std::env::current_exe().expect("Could not get executable path");
        let dir_path = exe_path
            .parent()
            .expect("Could not get executable directory");
        dir_path.join("tmgr_db")
    }

    /// Select a task from the database by a partial id.
    ///
    /// Returns a Task if exactly one task is found with the given id.
    /// Returns an error if no tasks are found, or if multiple tasks are found.
    ///
    /// The id should be a prefix of the full id of the task you want to select.
    /// The full id of each task is "task:<id>", where <id> is the id you
    /// provided when you added the task.
    pub(crate) async fn select_task_by_partial_id(
        &self,
        id: impl Into<String>,
    ) -> Result<Task, Box<dyn std::error::Error>> {
        let id_string = id.into();
        let query = format!(
            "SELECT * from task WHERE string::starts_with(<string> id, \"task:{}\")",
            &id_string
        );

        let res: Vec<Task> = self.client.query(query).await?.take(0)?;

        if res.is_empty() {
            return Err(format!("Task starting with id '{}' was not found", &id_string).into());
        }

        if res.len() != 1 {
            return Err("Multiple tasks found, provide more characters of the id"
                .to_string()
                .into());
        }

        let task = res.into_iter().next().expect("Expected a task");
        Ok(task)
    }
}
