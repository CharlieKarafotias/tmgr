use crate::cli::model::{Cli, Command};
use crate::commands;
use clap::Parser;

pub fn run() {
    let input = Cli::parse();

    match input.command {
        Command::Add {
            name,
            priority,
            description,
        } => {
            commands::add::run(name, priority, description);
        }
        Command::Complete { id } => {
            commands::complete::run(id);
        }
        Command::Delete { id } => {
            commands::delete::run(id);
        }
        Command::List { all } => {
            commands::list::run(all);
        }
        Command::Status => {
            commands::status::run();
        }
        Command::Update {
            id,
            name,
            priority,
            description,
        } => {
            commands::update::run(id, name, priority, description);
        }
        Command::Upgrade => {
            commands::upgrade::run();
        }
        Command::View { id } => {
            commands::view::run(id);
        }
    }
}
