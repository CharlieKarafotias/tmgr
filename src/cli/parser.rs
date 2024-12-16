use crate::cli::model::{Cli, Command};
use crate::cli::result_handler::handle_result;
use crate::commands;
use clap::Parser;

#[tokio::main]
pub async fn run() -> i32 {
    let input = Cli::parse();
    let db = if cfg!(test) {
        commands::db::DB::new_test().await
    } else {
        commands::db::DB::new().await
    };

    let res: Result<String, Box<dyn std::error::Error>> = match input.command {
        Command::Add {
            name,
            priority,
            description,
        } => commands::add::run(&db, name, priority, description).await,
        Command::Complete { id } => commands::complete::run(&db, id).await,
        Command::Delete { id } => commands::delete::run(&db, id).await,
        Command::List { all } => commands::list::run(&db, all).await,
        Command::Note { id, open } => commands::note::run(&db, id, open).await,
        Command::Status => commands::status::run(&db).await,
        Command::Tui => commands::tui::run(&db).await,
        Command::Update {
            id,
            name,
            priority,
            description,
        } => commands::update::run(&db, id, name, priority, description).await,
        Command::Upgrade => commands::upgrade::run().await,
        Command::View { id } => commands::view::run(&db, id).await,
    };

    let result = handle_result(res).await;
    println!("{}", result.result_string);
    result.exit_code
}
