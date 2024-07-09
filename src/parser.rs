mod commands;
mod state_mgr;
use clap::{Parser, Subcommand, ValueEnum};
use commands::{db_cmds, init_cmds, status_cmds, todo_cmds, update_cmds, TmgrResult};
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
        #[arg(short, long)]
        /// A short description of the task
        name: Option<String>,
        #[arg(short, long)]
        /// The priority of the task
        priority: Option<TaskPriority>,
        #[arg(short, long)]
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
    match State::new(None) {
        Ok(s) => run_with_state(s, input),
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

fn run_with_state(mut state: State, input: Cli) {
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
        } => init_cmds::initialize(&mut state).map_err(|e| e.into()),
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

// --- Integration Tests ---
#[cfg(test)]
mod cli_integration_tests {
    use assert_cmd::prelude::*;
    use std::process::Command;
    const DATABASE_NAME: &str = "db_for_integration_tests";

    // Util functions for integration tests
    fn setup_db() -> Result<(), Box<dyn std::error::Error>> {
        // Run init command to create database
        let mut init_cmd: Command = Command::cargo_bin("tmgr")?;
        init_cmd.arg("init");
        init_cmd.assert().success();

        // Run command to create integration test database
        let mut add_test_db = Command::cargo_bin("tmgr")?;
        add_test_db.arg("database").arg("add").arg(DATABASE_NAME);
        add_test_db.assert().success();

        // Run command to set active database to integration test database
        let mut set_test_db = Command::cargo_bin("tmgr")?;
        set_test_db.arg("database").arg("set").arg(DATABASE_NAME);
        set_test_db.assert().success();

        Ok(())
    }

    fn teardown_db() -> Result<(), Box<dyn std::error::Error>> {
        // Run command to delete integration test database
        let mut add_test_db = Command::cargo_bin("tmgr")?;
        add_test_db.arg("database").arg("delete").arg(DATABASE_NAME);
        add_test_db.assert().success();
        Ok(())
    }

    fn add_task() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("add").arg("new task created");
        cmd.assert().success();
        Ok(())
    }

    // --- Tests for todo command ---
    // TODO: add for all commands; part of issue [#23](https://github.com/CharlieKarafotias/tmgr/issues/23)
    #[test]
    fn test_todo_list_all() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list").arg("--all");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("ID"));
        cmd.assert().stdout(predicates::str::contains("Name"));
        teardown_db()?;
        Ok(())
    }

    #[test]
    fn test_todo_list() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("ID"));
        cmd.assert().stdout(predicates::str::contains("Name"));
        teardown_db()?;
        Ok(())
    }

    // --- todo update command tests ---
    #[test]
    fn test_todo_update_should_error_if_task_does_not_exist(
    ) -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1000")
            .args(["-n", "new name"]);
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("ERROR"));
        cmd.assert().stdout(predicates::str::contains("not found"));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_name_short() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        add_task()?;
        let task_name = "test test_todo_update_should_update_name_short updated name successfully";
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1")
            .arg("-n")
            .arg(task_name);
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains(task_name));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_name_long() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        add_task()?;
        let task_name = "test test_todo_update_should_update_name_long updated name successfully";
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1")
            .arg("--name")
            .arg(task_name);
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains(task_name));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_priority_short() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        add_task()?;
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("update").arg("1").arg("-p").arg("low");
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("low"));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_priority_long() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        add_task()?;
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1")
            .arg("--priority")
            .arg("medium");
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert().stdout(predicates::str::contains("medium"));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_description_short() -> Result<(), Box<dyn std::error::Error>>
    {
        setup_db()?;
        add_task()?;
        let task_description =
            "test test_todo_update_should_update_description_short updated description successfully";
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1")
            .arg("-d")
            .arg(task_description);
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains(task_description));
        teardown_db()?;
        Ok(())
    }

    #[test]
    #[ignore = "reason: not implemented yet"]
    fn test_todo_update_should_update_description_long() -> Result<(), Box<dyn std::error::Error>> {
        setup_db()?;
        add_task()?;
        let task_description =
            "test test_todo_update_should_update_description_long updated description successfully";
        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo")
            .arg("update")
            .arg("1")
            .arg("--description")
            .arg(task_description);
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains("Successfully updated task"));

        let mut cmd = Command::cargo_bin("tmgr")?;
        cmd.arg("todo").arg("list");
        cmd.assert().success();
        cmd.assert()
            .stdout(predicates::str::contains(task_description));
        teardown_db()?;
        Ok(())
    }
}
