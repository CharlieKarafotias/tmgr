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

    let res: Result<String, Box<dyn std::error::Error>> = match input.command {
        Command::Add {
            name,
            priority,
            description,
        } => commands::add::run(&db, name, priority, description).await,
        Command::Complete { name } => commands::complete::run(&db, name).await,
        Command::Delete { id } => commands::delete::run(&db, id).await,
        Command::List { all } => commands::list::run(&db, all).await,
        Command::Status => commands::status::run(&db).await,
        Command::Update {
            id,
            name,
            priority,
            description,
        } => commands::update::run(&db, id, name, priority, description).await,
        Command::Upgrade => commands::upgrade::run().await,
        Command::View { id } => commands::view::run(&db, id).await,
    };

    if res.is_err() {
        println!("{:?}", res);
    } else {
        println!("{}", res.unwrap());
    }
}
