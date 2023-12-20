use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Task {
    id: i64,
    name: String,
    priority: String,
    description: Option<String>,
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
                description TEXT
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
        let sql = "INSERT INTO tasks (name, priority, description) VALUES (?, ?, ?)";
        let mut stmt = self.conn.prepare(sql)?;
        let res = stmt.execute(params![name, priority, description])?;
        Ok(res)
        // Insert a new task
    }

    pub fn list_tasks(&self) -> Result<()> {
        let sql = "SELECT * FROM tasks";
        let mut stmt = self.conn.prepare(sql)?;

        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                priority: row.get(2)?,
                description: row.get(3)?,
            })
        })?;
        let mut tasks = Vec::new();
        for row_result in rows {
            tasks.push(row_result?);
        }
        // Print the header
        println!(
            "{:<5} {:<20} {:<10} {:<20}",
            "ID", "Name", "Priority", "Description"
        );
        println!("{}", "-".repeat(60));

        // // Print each task
        for task in tasks {
            println!(
                "{:<5} {:<20} {:<10} {:<20}",
                task.id,
                task.name,
                task.priority,
                task.description.unwrap_or_else(|| "".to_string()),
            );
        }

        println!("{}", "-".repeat(60));

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
}
