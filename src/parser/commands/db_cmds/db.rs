use super::{DatabaseError, DatabaseErrorKind, State};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::env;
use surrealdb::{
    engine::any::{self, Any},
    sql::Thing,
    Surreal,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub priority: String,
    pub description: Option<String>,
    pub created_on: DateTime<Local>,
    pub completed_on: Option<DateTime<Local>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskWithId {
    pub id: Thing,
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
        db = any::connect(format!("file://{}", db_directory))
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
            .content(Task {
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

    // TODO: finish these two then fix other errors then add tests
    pub fn complete_todo(&self, id: i64) -> Result<usize, DatabaseError> {
        let sql = "UPDATE tasks SET completed_on = (?) WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;
        let rows_updated = stmt
            .execute(params![Utc::now(), id])
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            })?;
        match rows_updated {
            0 => Err(DatabaseError {
                kind: DatabaseErrorKind::EntryNotFound,
                message: format!("Task with id {id} not found"),
            }),
            rows_updated => Ok(rows_updated),
        }
    }

    pub fn update_todo(
        &self,
        id: i64,
        new_name: Option<String>,
        new_priority: Option<String>,
        new_description: Option<String>,
    ) -> Result<usize, DatabaseError> {
        let mut update_sql = "UPDATE tasks SET".to_string();

        // TODO: definitely a better way to add in these param_values, maybe a map on Some values instead of if lets
        let mut param_values: Vec<rusqlite::types::Value> = Vec::new();

        if let Some(name) = new_name {
            update_sql.push_str(" name = (?),");
            param_values.push(name.clone().into());
        }
        if let Some(priority) = new_priority {
            update_sql.push_str(" priority = (?),");
            param_values.push(priority.clone().into());
        }
        if let Some(description) = new_description {
            update_sql.push_str(" description = (?),");
            param_values.push(description.clone().into());
        }

        // remove trailing comma
        update_sql.pop();
        update_sql.push_str(" WHERE id = (?);");
        param_values.push(id.into());

        // TODO: in future, replace ? (error propagation) with actual error and wrap with one of the db_errors defined in db_errors.rs
        let rows_updated = self
            .conn
            .execute(&update_sql, rusqlite::params_from_iter(param_values))
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            })?;

        match rows_updated {
            0 => Err(DatabaseError {
                kind: DatabaseErrorKind::EntryNotFound,
                message: format!("Task with id {id} not found"),
            }),
            rows_updated => Ok(rows_updated),
        }
    }
}
