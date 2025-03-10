use super::super::model::TaskPriority;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    author = "Charlie Karafotias",
    version,
    about = "Store todo tasks",
    propagate_version = true
)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Command,
}

#[derive(Subcommand, Debug)]
pub(super) enum Command {
    /// Add a new task
    Add {
        /// A short description of the task
        name: String,
        #[arg(short, long)]
        /// An optional priority of the task (will use low priority by default)
        priority: Option<TaskPriority>,
        #[arg(short, long)]
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
    /// Migrate will migrate the database from an older version of tmgr to be compatible with the latest version
    Migrate {
        /// The major version of the database to migrate from
        from: TmgrVersion,
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

#[derive(Clone, Debug, ValueEnum)]
pub(crate) enum TmgrVersion {
    V2,
    V3,
    Invalid,
}

impl From<u32> for TmgrVersion {
    fn from(value: u32) -> Self {
        match value {
            2 => TmgrVersion::V2,
            3 => TmgrVersion::V3,
            _ => TmgrVersion::Invalid,
        }
    }
}
impl From<&TmgrVersion> for String {
    fn from(value: &TmgrVersion) -> Self {
        match value {
            TmgrVersion::V2 => "v2".to_string(),
            TmgrVersion::V3 => "v3".to_string(),
            TmgrVersion::Invalid => "invalid".to_string(),
        }
    }
}
