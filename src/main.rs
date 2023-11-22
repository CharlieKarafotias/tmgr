use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(author = "Charlie Karafotias", version, about = "Store todo tasks", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Database,
    Status,
    Todo {
        #[command(subcommand)]
        command: Todo,
    },
}

#[derive(Debug, Subcommand)]
enum Todo {
    Add {
        /// A short description of the task
        name: String,
        /// The priority of the task
        #[clap(default_value_t = TaskPriority::High)]
        priority: TaskPriority,
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

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
