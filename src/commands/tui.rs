mod app;
mod ui;

use crate::{cli::model::CommandResult, commands::db::DB};
use app::App;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::{error::Error, io};

pub(crate) async fn run(db: &DB) -> Result<CommandResult<()>, Box<dyn Error>> {
    init_panic_hook();
    let mut terminal = init_tui()?;

    let mut app = App::new(db).await?;
    app.run(&mut terminal).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen,)?;

    Ok(CommandResult::new("".to_string(), ()))
}

fn init_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

fn init_tui() -> io::Result<Terminal<impl Backend>> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(io::stdout()))
}

fn restore_tui() -> Result<CommandResult<()>, Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(CommandResult::new("".to_string(), ()))
}
