use super::{DatabaseError, DatabaseErrorKind, State};
use chrono::Utc;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct TaskFromDb {
    pub id: i64,
    pub name: String,
    pub priority: String,
    pub description: Option<String>,
    pub created_on: String,
    pub completed_on: Option<String>,
}

pub struct DB {
    conn: Connection,
}

// TODO: this file should be more abstracted, CRUD like using the structs defined for Todo
impl DB {
    pub fn new(state: &State) -> Result<Self, DatabaseError> {
        let curr_db = state.get_db_var().unwrap_or("none".to_string());
        if curr_db == "none" {
            return Err(DatabaseError {
                kind: DatabaseErrorKind::VariableNotSet,
                message: "No database selected".to_string(),
            });
        }
        let path = state.get_db_var_full_path().ok_or(DatabaseError {
            kind: DatabaseErrorKind::PathCreationFailed,
            message: "State manager failed to create path to database file".to_string(),
        })?;
        // Open or create a SQLite database file
        let conn = Connection::open(path).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;
        // Create a 'tasks' table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                priority TEXT NOT NULL,
                description TEXT,
                created_on TEXT NOT NULL,
                completed_on TEXT
            )",
            [],
        )
        .map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;
        Ok(DB { conn })
    }

    pub fn add_task(
        &self,
        name: String,
        priority: String,
        description: Option<String>,
    ) -> Result<usize, DatabaseError> {
        let sql = "INSERT INTO tasks (name, priority, description, created_on) VALUES (?, ?, ?, ?)";
        let mut stmt = self.conn.prepare(sql).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;
        let rows_updated = stmt
            .execute(params![name, priority, description, Utc::now()])
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            })?;
        Ok(rows_updated)
    }

    pub fn count_tasks(&self) -> Result<usize, DatabaseError> {
        let sql = "SELECT COUNT(name) FROM tasks";
        let res = self.conn.query_row(sql, [], |r| r.get(0));
        match res {
            Ok(r) => Ok(r),
            Err(e) => Err(DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            }),
        }
    }

    pub fn list_tasks(&self) -> Result<Vec<TaskFromDb>, DatabaseError> {
        let sql = "SELECT * FROM tasks";
        let mut stmt = self.conn.prepare(sql).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;

        let rows = stmt
            .query_map([], |row| {
                Ok(TaskFromDb {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    priority: row.get(2)?,
                    description: row.get(3)?,
                    created_on: row.get(4)?,
                    completed_on: row.get(5)?,
                })
            })
            .map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            })?;
        let mut tasks = Vec::new();
        for row_result in rows {
            let res = row_result.map_err(|e| DatabaseError {
                kind: DatabaseErrorKind::SQLiteError,
                message: e.to_string(),
            })?;
            tasks.push(res);
        }
        Ok(tasks)
    }

    pub fn delete_todo(&self, id: i64) -> Result<usize, DatabaseError> {
        let sql = "DELETE FROM tasks WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql).map_err(|e| DatabaseError {
            kind: DatabaseErrorKind::SQLiteError,
            message: e.to_string(),
        })?;
        let rows_updated = stmt.execute(params![id]).map_err(|e| DatabaseError {
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
