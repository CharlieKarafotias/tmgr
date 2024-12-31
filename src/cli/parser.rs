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

    let result = match input.command {
        Command::Add {
            name,
            priority,
            description,
        } => {
            let res = commands::add::run(&db, name, priority, description).await;
            handle_result(res).await
        }
        Command::Complete { id } => {
            let res = commands::complete::run(&db, id).await;
            handle_result(res).await
        }
        Command::Delete { id } => {
            let res = commands::delete::run(&db, id).await;
            handle_result(res).await
        }
        Command::List { all } => {
            let res = commands::list::run(&db, all).await;
            handle_result(res).await
        }
        Command::Note { id, open } => {
            let res = commands::note::run(&db, id, open).await;
            handle_result(res).await
        }
        Command::Status => {
            let res = commands::status::run(&db).await;
            handle_result(res).await
        }
        Command::Tui => {
            let res = commands::tui::run(&db).await;
            handle_result(res).await
        }
        Command::Update {
            id,
            name,
            priority,
            description,
        } => {
            let res = commands::update::run(&db, id, name, priority, description).await;
            handle_result(res).await
        }
        Command::Upgrade => {
            let res = commands::upgrade::run().await;
            handle_result(res).await
        }
        Command::View { id } => {
            let res = commands::view::run(&db, id).await;
            handle_result(res).await
        }
    };

    println!("{}", result.result_string);
    result.exit_code
}
