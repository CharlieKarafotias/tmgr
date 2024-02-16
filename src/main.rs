mod db;
mod db_cmds;
mod persistent;

use clap::{Parser, Subcommand, ValueEnum};

use crate::{
    db::DB,
    db_cmds::{db_add, db_delete, db_list, db_set},
};

#[derive(Debug, Parser)]
#[command(author = "Charlie Karafotias", version, about = "Store todo tasks", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Database {
        #[command(subcommand)]
        command: Database,
    },
    /// Info regarding file locations, current database, general statistics
    Status,
    Todo {
        #[command(subcommand)]
        command: Todo,
    },
}

// Declarations for the Todo command
#[derive(Debug, Subcommand)]
/// Access commands related to a particular task
enum Todo {
    /// Add a new task
    Add {
        /// A short description of the task
        name: String,
        /// The priority of the task
        #[clap(default_value_t = TaskPriority::High)]
        priority: TaskPriority,
        /// An optional long description of the task
        description: Option<String>,
    },
    /// Mark a task as complete
    Complete {
        /// The id of the task
        id: i64,
    },
    /// Delete a task
    Delete {
        /// The id of the task
        id: i64,
    },
    /// List all tasks
    List,
    /// Update an existing task
    Update {
        /// The id of the task
        id: i64,
        /// A short description of the task
        name: Option<String>,
        /// The priority of the task
        priority: Option<TaskPriority>,
        /// An optional long description of the task
        description: Option<String>,
    },
}

#[derive(Clone, Debug, ValueEnum)]
enum TaskPriority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

// Declarations for the Database command
#[derive(Debug, Subcommand)]
/// Access database commands
enum Database {
    /// Add a new database
    Add {
        /// The database name
        name: String,
    },
    /// Delete an existing database
    Delete {
        /// The database name
        name: String,
    },
    /// List all databases
    List,
    /// Set the working database
    Set {
        /// The database name
        name: String,
    },
}
fn main() {
    let cli = Cli::parse();
    match cli {
        Cli {
            command: Command::Database {
                command: db_command,
            },
        } => match db_command {
            Database::Add { name } => db_add(name),
            Database::Delete { name } => db_delete(name),
            Database::List => db_list(),
            Database::Set { name } => db_set(name),
        },
        Cli {
            command: Command::Status,
        } => {
            todo!("Not implemented");
        }
        Cli {
            command: Command::Todo {
                command: todo_command,
            },
        } => match todo_command {
            Todo::Add {
                name,
                priority,
                description,
            } => {
                let db = DB::new().unwrap();
                let res = db.add_task(name, priority.to_string(), description);
                println!("{:?}", res);
            }
            Todo::Complete { id } => {
                let db = DB::new().unwrap();
                db.complete_todo(id);
            }
            Todo::Delete { id } => {
                let db = DB::new().unwrap();
                db.delete_todo(id);
            }
            Todo::List => {
                let db = DB::new().unwrap();
                db.list_tasks().unwrap();
            }
            Todo::Update {
                id,
                name,
                priority,
                description,
            } => {
                let db = DB::new().unwrap();
                let mut priorityStr: Option<String> = None;
                if let Some(priority) = priority {
                    priorityStr = Some(priority.to_string());
                }
                let _ = db.update_todo(id, name, priorityStr, description);
            }
        },
    }
}
