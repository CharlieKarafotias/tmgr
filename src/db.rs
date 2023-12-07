use rusqlite::{Connection, Result};

pub struct DB {
    conn: Connection,
}

impl DB {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("tasks.db")?; // Open or create a SQLite database file
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL
            )",
            [],
        )?; // Create a 'tasks' table if it doesn't exist

        Ok(DB { conn })
    }

    pub fn add_task(&self, description: &str) -> Result<usize> {
        self.conn
            .execute("INSERT INTO tasks (description) VALUES (?)", &[description])
        // Insert a new task
    }

    pub fn list_tasks(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare("SELECT description FROM tasks")?; // Retrieve all tasks
        let task_iter = stmt.query_map([], |row| row.get(0))?;
        let mut tasks = Vec::new();

        for task in task_iter {
            tasks.push(task?);
        }

        Ok(tasks)
    }
}
