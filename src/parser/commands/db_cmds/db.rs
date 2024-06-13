use super::{db_errors, State};
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
    pub fn new(state: &State) -> Result<Self, Box<dyn std::error::Error>> {
        let curr_db = state.get_db_var().unwrap_or("none".to_string());
        if curr_db == "none" {
            return Err(db_errors::DatabaseError::new(
                "No database selected",
                db_errors::DatabaseErrorKind::VariableNotSet,
            )
            .into());
        }
        match state.get_db_var_full_path() {
            Some(path) => {
                let conn = Connection::open(path)?; // Open or create a SQLite database file
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
                )?; // Create a 'tasks' table if it doesn't exist

                Ok(DB { conn })
            }
            None => Err(db_errors::DatabaseError::new(
                "State manager failed to create path to database file",
                db_errors::DatabaseErrorKind::PathCreationFailed,
            )
            .into()),
        }
    }

    pub fn add_task(
        &self,
        name: String,
        priority: String,
        description: Option<String>,
    ) -> Result<usize> {
        let sql = "INSERT INTO tasks (name, priority, description, created_on) VALUES (?, ?, ?, ?)";
        let mut stmt = self.conn.prepare(sql)?;
        let rows_updated = stmt.execute(params![name, priority, description, Utc::now()])?;
        Ok(rows_updated)
    }

    pub fn count_tasks(&self) -> Result<usize, Box<dyn std::error::Error>> {
        let sql = "SELECT COUNT(name) FROM tasks";
        let res = self.conn.query_row(sql, [], |r| r.get(0));
        match res {
            Ok(r) => Ok(r),
            Err(e) => Err(Box::new(e)),
        }
    }

    // TODO: this function should return type Result<Vec<TaskFromDb>, Box<dyn std::error::Error>> instead and leave printing to todo_cmds.rs file
    pub fn list_tasks(&self) -> Result<Vec<TaskFromDb>> {
        let sql = "SELECT * FROM tasks";
        let mut stmt = self.conn.prepare(sql)?;

        let rows = stmt.query_map([], |row| {
            Ok(TaskFromDb {
                id: row.get(0)?,
                name: row.get(1)?,
                priority: row.get(2)?,
                description: row.get(3)?,
                created_on: row.get(4)?,
                completed_on: row.get(5)?,
            })
        })?;
        let mut tasks = Vec::new();
        for row_result in rows {
            tasks.push(row_result?);
        }
        Ok(tasks)
    }

    pub fn delete_todo(&self, id: i64) -> Result<usize, Box<dyn std::error::Error>> {
        let sql = "DELETE FROM tasks WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql)?;
        let rows_updated = stmt.execute(params![id])?;
        match rows_updated {
            0 => Err(Box::new(db_errors::DatabaseError::new(
                format!("Task with id {id} not found").as_str(),
                db_errors::DatabaseErrorKind::EntryNotFound,
            ))),
            rows_updated => Ok(rows_updated),
        }
    }

    pub fn complete_todo(&self, id: i64) -> Result<usize, Box<dyn std::error::Error>> {
        let sql = "UPDATE tasks SET completed_on = (?) WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql)?;
        let rows_updated = stmt.execute(params![Utc::now(), id])?;
        match rows_updated {
            0 => Err(Box::new(db_errors::DatabaseError::new(
                format!("Task with id {id} not found").as_str(),
                db_errors::DatabaseErrorKind::EntryNotFound,
            ))),
            rows_updated => Ok(rows_updated),
        }
    }

    pub fn update_todo(
        &self,
        id: i64,
        new_name: Option<String>,
        new_priority: Option<String>,
        new_description: Option<String>,
    ) -> Result<usize, Box<dyn std::error::Error>> {
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
            .execute(&update_sql, rusqlite::params_from_iter(param_values))?;

        match rows_updated {
            0 => Err(Box::new(db_errors::DatabaseError::new(
                format!("Task with id {id} not found").as_str(),
                db_errors::DatabaseErrorKind::EntryNotFound,
            ))),
            rows_updated => Ok(rows_updated),
        }
    }
}
