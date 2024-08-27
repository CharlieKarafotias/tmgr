use crate::cli::model::{Cli, Command};
use crate::commands;
use clap::Parser;

#[tokio::main]
pub async fn run() {
    let input = Cli::parse();
    #[allow(clippy::needless_late_init)]
    let db: commands::db::DB;
    if cfg!(test) {
        db = commands::db::DB::new_test().await;
    } else {
        db = commands::db::DB::new().await;
    }

    match input.command {
        Command::Add {
            name,
            priority,
            description,
        } => {
            commands::add::run(&db, name, priority, description).await;
        }
        Command::Complete { id } => {
            commands::complete::run(&db, id).await;
        }
        Command::Delete { id } => {
            commands::delete::run(&db, id).await;
        }
        Command::List { all } => {
            commands::list::run(&db, all).await;
        }
        Command::Status => {
            commands::status::run(&db).await;
        }
        Command::Update {
            id,
            name,
            priority,
            description,
        } => {
            commands::update::run(&db, id, name, priority, description).await;
        }
        Command::Upgrade => {
            commands::upgrade::run().await;
        }
        Command::View { id } => {
            commands::view::run(&db, id).await;
        }
    }
}