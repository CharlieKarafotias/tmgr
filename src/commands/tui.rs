mod app;
mod ui;

use crate::{cli::model::CommandResult, commands::db::DB};
use app::App;
use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    Terminal,
};
use std::{error::Error, io};

pub(crate) async fn run(db: &DB) -> Result<CommandResult<()>, Box<dyn Error>> {
    // start terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new(db).await?;
    let _res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen,)?;

    Ok(CommandResult::new("".to_string(), ()))
}
