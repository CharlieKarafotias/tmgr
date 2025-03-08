use super::super::{
    cli::{
        model::{Cli, Command},
        result_handler::handle_result,
    },
    commands,
    db::DB,
    model::TmgrError,
};
use clap::Parser;

#[tokio::main]
pub async fn run() -> i32 {
    let input = Cli::parse();
    let db = if cfg!(test) {
        DB::new_test().await
    } else {
        DB::new().await
    };

    let res: Result<String, TmgrError> = match db {
        Err(e) => Err(TmgrError::from(e)),
        Ok(db) => match input.command {
            Command::Add {
                name,
                priority,
                description,
            } => commands::add::run(&db, name, priority, description)
                .await
                .map_err(TmgrError::from),
            Command::Complete { id } => commands::complete::run(&db, id)
                .await
                .map_err(TmgrError::from),
            Command::Delete { id } => commands::delete::run(&db, id)
                .await
                .map_err(TmgrError::from),
            Command::List { all } => commands::list::run(&db, all).await.map_err(TmgrError::from),
            Command::Migrate { from } => commands::migrate::run(&db, from)
                .await
                .map_err(TmgrError::from),
            Command::Note { id, open } => commands::note::run(&db, id, open)
                .await
                .map_err(TmgrError::from),
            Command::Status => commands::status::run(&db).await.map_err(TmgrError::from),
            Command::Update {
                id,
                name,
                priority,
                description,
            } => commands::update::run(&db, id, name, priority, description)
                .await
                .map_err(TmgrError::from),
            Command::Upgrade => commands::upgrade::run().await.map_err(TmgrError::from),
            Command::View { id } => commands::view::run(&db, id).await.map_err(TmgrError::from),
        },
    };

    let result = handle_result(res).await;
    println!("{}", result.result_string());
    result.exit_code()
}
