use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author = "Charlie Karafotias", version, about = "Store todo tasks", long_about = None)]
#[command(propagate_version = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
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
        /// The name of the task to update
        name: String,
    },
    /// Delete a task
    Delete {
        /// The name of the task to delete
        name: String,
    },
    /// List all tasks. By default, this will only list in-progress tasks.
    List {
        #[arg(short, long)]
        /// List all tasks, including completed ones
        all: bool,
    },
    /// Info regarding file locations, current database, general statistics
    Status,
    /// Update a task
    Update {
        /// The current name of the task to update
        current_name: String,
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
    /// Upgrade to the latest version
    Upgrade,
    /// View a specific task
    View {
        /// The id of the task
        id: String,
    },
}

#[derive(Clone, ValueEnum, Debug)]
pub(crate) enum TaskPriority {
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
