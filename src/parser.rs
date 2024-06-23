mod commands;
mod state_mgr;
use clap::{Parser, Subcommand, ValueEnum};
use commands::{db_cmds, status_cmds, todo_cmds, update_cmds, TmgrResult};
use state_mgr::State;

// Clap structs (CLI constructs)
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
    /// Initializes tmgr for use
    Init,
    /// Info regarding file locations, current database, general statistics
    Status,
    Todo {
        #[command(subcommand)]
        command: Todo,
    },
    /// Update tmgr to the latest stable version
    Update,
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
    /// List the tasks in the database. By default, this will only list in progress tasks.
    List {
        #[arg(short, long)]
        /// List all tasks, including completed ones
        all: bool,
    },
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
    /// Set the directory where databases are stored
    SetDirectory {
        /// The directory path
        path: String,
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

// Calls the CLI and runs the correct command
pub fn run_cli() {
    let input = Cli::parse();
    let mut state = State::new(None);
    let res: TmgrResult<()> = match input {
        Cli {
            command: Command::Database {
                command: db_command,
            },
        } => match db_command {
            Database::Add { name } => db_cmds::db_add(&mut state, name).map_err(|e| e.into()),
            Database::Delete { name } => db_cmds::db_delete(&mut state, name).map_err(|e| e.into()),
            Database::List => db_cmds::db_list(&mut state).map_err(|e| e.into()),
            Database::Set { name } => db_cmds::db_set(&mut state, name).map_err(|e| e.into()),
            Database::SetDirectory { path } => {
                db_cmds::db_set_directory(&mut state, path).map_err(|e| e.into())
            }
        },
        Cli {
            command: Command::Status,
        } => status_cmds::get_status(&state).map_err(|e| e.into()),
        Cli {
            command: Command::Init,
        } => {
            let db_name = "init_db".to_string();
            let _ = db_cmds::db_set_directory(&mut state, ".".to_string());
            let _ = db_cmds::db_add(&mut state, db_name.clone());
            let _ = db_cmds::db_set(&mut state, db_name);
            println!("Initializer complete!");
            Ok(())
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
            } => todo_cmds::add(&mut state, name, priority, description).map_err(|e| e.into()),
            Todo::Complete { id } => todo_cmds::complete(&mut state, id).map_err(|e| e.into()),
            Todo::Delete { id } => todo_cmds::delete(&mut state, id).map_err(|e| e.into()),
            Todo::List { all } => todo_cmds::list(&mut state, all).map_err(|e| e.into()),
            Todo::Update {
                id,
                name,
                priority,
                description,
            } => {
                todo_cmds::update(&mut state, id, name, priority, description).map_err(|e| e.into())
            }
        },
        Cli {
            command: Command::Update,
        } => update_cmds::update(&state).map_err(|e| e.into()),
    };
    if let Err(e) = res {
        println!("ERROR: {}", e)
    }
}
