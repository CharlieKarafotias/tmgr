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
        /// The id of the task to update (can be partial)
        id: String,
    },
    /// Delete a task
    Delete {
        /// The id of the task to delete (can be partial)
        id: String,
    },
    /// List all tasks. By default, this will only list in-progress tasks.
    List {
        #[arg(short, long)]
        /// List all tasks, including completed ones
        all: bool,
    },
    /// Creates and/or opens a markdown file to store notes associated with a particular task
    Note {
        /// The id of the task (can be partial)
        id: String,
        #[arg(short, long)]
        /// Opens up file in vi editor
        open: bool,
    },
    /// Info regarding file locations, current database, general statistics
    Status,
    /// Launch the terminal user interface.
    Tui,
    /// Update a task
    Update {
        /// The id of the task to update (can be partial)
        id: String,
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
        /// The id of the task to view (can be partial)
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

#[derive(Clone, Debug)]
pub(crate) struct CommandResult<T> {
    message: String,
    result: T,
}

impl<T> CommandResult<T> {
    pub(crate) fn new(message: String, result: T) -> Self {
        Self { message, result }
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn result(&self) -> &T {
        &self.result
    }
}
