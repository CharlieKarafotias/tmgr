mod db_ops;
use std::path::Path;

use clap::{Parser, Subcommand, ValueEnum};
use db_ops::establish_connection;

use crate::db_ops::remove_db;

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
    Status,
    Todo {
        #[command(subcommand)]
        command: Todo,
    },
}

// Declarations for the Todo command
#[derive(Debug, Subcommand)]
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
        id: i32,
    },
    /// Delete a task
    Delete {
        /// The id of the task
        id: i32,
    },
    /// List all tasks
    List,
    /// Update an existing task
    Update {
        /// The id of the task
        id: i32,
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
        } => {
            println!("db command entered");
            match db_command {
                Database::Add { name } => {
                    let path = format!("./{name}.db");
                    establish_connection(Path::new(&path)).expect("Failed to create new database");
                }
                Database::Delete { name } => {
                    let path = format!("./{name}.db");
                    remove_db(Path::new(&path)).expect("Failed to delete database");
                }
                Database::List => todo!(),
                Database::Set { name } => todo!(),
            }
        }
        Cli {
            command: Command::Status,
        } => {
            println!("status command entered");
        }
        Cli {
            command: Command::Todo {
                command: todo_command,
            },
        } => {
            println!("todo command entered");
            match todo_command {
                Todo::Add {
                    name,
                    priority,
                    description,
                } => todo!(),
                Todo::Complete { id } => todo!(),
                Todo::Delete { id } => todo!(),
                Todo::List => todo!(),
                Todo::Update {
                    id,
                    name,
                    priority,
                    description,
                } => todo!(),
            }
        }
    }
    // println!("{:?}", cli);
    // let conn = establish_connection(Path::new("./test.db")).unwrap();

    // println!("{}", table_exists(conn, "tasks").expect("error"));
}
