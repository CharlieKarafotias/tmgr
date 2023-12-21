use chrono::Utc;
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct TaskFromDb {
    id: i64,
    name: String,
    priority: String,
    description: Option<String>,
    created_on: String,
    completed_on: Option<String>,
}

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("tasks.db")?; // Open or create a SQLite database file
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

    pub fn add_task(
        &self,
        name: String,
        priority: String,
        description: Option<String>,
    ) -> Result<usize> {
        let sql = "INSERT INTO tasks (name, priority, description, created_on) VALUES (?, ?, ?, ?)";
        let mut stmt = self.conn.prepare(sql)?;
        let res = stmt.execute(params![name, priority, description, Utc::now()])?;
        Ok(res)
        // Insert a new task
    }

    pub fn list_tasks(&self) -> Result<()> {
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
        // Print the header
        println!(
            "{:<5} {:<20} {:<10} {:<20} {:<35} {:<35}",
            "ID", "Name", "Priority", "Description", "Created On", "Completed On"
        );
        println!("{}", "-".repeat(110));

        // // Print each task
        for task in tasks {
            println!(
                "{:<5} {:<20} {:<10} {:<20} {:<35} {:<35}",
                task.id,
                task.name,
                task.priority,
                task.description.unwrap_or_else(|| "NULL".to_string()),
                task.created_on,
                task.completed_on
                    .unwrap_or_else(|| "IN-PROGRESS".to_string())
            );
        }

        println!("{}", "-".repeat(110));

        Ok(())
    }

    pub fn delete_todo(&self, id: i64) -> Result<()> {
        let sql = "DELETE FROM tasks WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql)?;
        let res = stmt.execute(params![id])?;
        match res {
            0 => println!("Task with id {} not found", id),
            _ => println!("Successfully deleted task"),
        }
        Ok(())
    }

    pub fn complete_todo(&self, id: i64) -> Result<()> {
        let sql = "UPDATE tasks SET completed_on = (?) WHERE id = (?)";
        let mut stmt = self.conn.prepare(sql)?;
        let res = stmt.execute(params![Utc::now(), id])?;
        match res {
            0 => println!("Task with id {} not found", id),
            _ => println!("Successfully updated task"),
        }
        Ok(())
    }
}
