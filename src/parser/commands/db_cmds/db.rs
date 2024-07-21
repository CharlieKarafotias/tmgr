use super::{DatabaseError, DatabaseErrorKind, State};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::{
    engine::any::{self, Any},
    opt::PatchOp,
    sql::Thing,
    Surreal,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskWithId {
    pub id: Option<Thing>,
    pub name: String,
    pub priority: String,
    pub description: Option<String>,
    pub created_on: DateTime<Local>,
    pub completed_on: Option<DateTime<Local>>,
}

pub struct DB {
    conn: Surreal<Any>,
}

impl DB {
    pub async fn new(state: &State) -> Result<Self, DatabaseError> {
        let curr_db = state.get_db_name().unwrap_or("none".to_string());
        let db: Surreal<Any>;

        if env::var("TMGR_TEST").is_ok() {
            db = any::connect("memory").await.map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;

            return Ok(DB { conn: db });
        }

        // TODO: WRITE TEST FOR THIS
        if curr_db == "none" {
            return Err(DatabaseError {
                kind: DatabaseErrorKind::VariableNotSet,
                message: "No database selected".to_string(),
            });
        }

        let db_directory = state.get_db_dir().ok_or(DatabaseError {
            kind: DatabaseErrorKind::DirectoryNotSet,
            message: "No database directory".to_string(),
        })?;

        // Open or create database file
        db = any::connect(format!("file://{}.db", db_directory))
            .await
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;

        // Set correct namespace and database
        db.use_ns("tmgr")
            .use_db(curr_db)
            .await
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;

        Ok(DB { conn: db })
    }

    pub async fn add_task(
        &self,
        name: String,
        priority: String,
        description: Option<String>,
    ) -> Result<Vec<TaskWithId>, DatabaseError> {
        let res = self
            .conn
            .create("task")
            .content(TaskWithId {
                id: None,
                name,
                priority,
                description,
                created_on: Local::now(),
                completed_on: None,
            })
            .await
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;
        Ok(res)
    }

    pub async fn count_tasks(&self) -> Result<Option<i32>, DatabaseError> {
        let mut res = self.conn.query("task").await.map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SurrealDBError,
            message: e.to_string(),
        })?;
        let count: Option<i32> = res.take(0).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SurrealDBError,
            message: e.to_string(),
        })?;
        Ok(count)
    }

    pub async fn list_tasks(&self) -> Result<Vec<TaskWithId>, DatabaseError> {
        let res: Vec<TaskWithId> = self.conn.select("task").await.map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SurrealDBError,
            message: e.to_string(),
        })?;
        Ok(res)
    }

    pub async fn delete_todo(&self, id: String) -> Result<Vec<TaskWithId>, DatabaseError> {
        let res: Option<Vec<TaskWithId>> =
            self.conn
                .delete(("task", &id))
                .await
                .map_err(|e| DatabaseError {
                    kind: DatabaseErrorKind::SurrealDBError,
                    message: e.to_string(),
                })?;
        match res {
            None => Err(DatabaseError {
                kind: DatabaseErrorKind::EntryNotFound,
                message: format!("Task with id {id} not found"),
            }),
            Some(res) => Ok(res),
        }
    }

    pub async fn complete_todo(&self, id: String) -> Result<Vec<TaskWithId>, DatabaseError> {
        #[derive(Serialize)]
        struct TaskCompleted {
            completed_on: Option<DateTime<Local>>,
        }
        let res: Option<Vec<TaskWithId>> = self
            .conn
            .update(("task", &id))
            .patch(PatchOp::replace("/completed_on", Some(Local::now())))
            .await
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;
        match res {
            None => Err(DatabaseError {
                kind: DatabaseErrorKind::EntryNotFound,
                message: format!("Task with id {id} not found"),
            }),
            Some(res) => Ok(res),
        }
    }

    pub async fn update_todo(
        &self,
        id: String,
        new_name: Option<String>,
        new_priority: Option<String>,
        new_description: Option<String>,
    ) -> Result<Vec<TaskWithId>, DatabaseError> {
        // TODO: might be wrong, need to test
        let res: Option<Vec<TaskWithId>> = self
            .conn
            .update(("task", &id))
            .merge(TaskWithId {
                id: None,
                name: new_name.unwrap_or_default(),
                priority: new_priority.unwrap_or_default(),
                description: new_description,
                created_on: Local::now(),
                completed_on: None, // TODO: definitely wrong
            })
            .await
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SurrealDBError,
                message: e.to_string(),
            })?;
        match res {
            None => Err(DatabaseError {
                kind: DatabaseErrorKind::EntryNotFound,
                message: format!("Task with id {id} not found"),
            }),
            Some(res) => Ok(res),
        }
    }
}
