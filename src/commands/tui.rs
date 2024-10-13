use crate::commands::db::DB;
use crate::commands::model::Task;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::{Constraint, Direction, Layout, Style},
    style::Stylize,
    widgets::{Block, HighlightSpacing, List, ListState, Paragraph},
    Terminal,
};
use std::cmp::{max, min};
use std::io::stdout;

pub(crate) async fn run(db: &DB) -> Result<String, Box<dyn std::error::Error>> {
    println!("Starting TUI...");
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    let mut list_state = ListState::default();
    let tasks: Vec<Task> = db.client.select("task").await?;
    if !tasks.is_empty() {
        list_state.select(Some(0));
    }
    loop {
        terminal.draw(|f| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(f.area());
            f.render_widget(
                Paragraph::new("Task Manager").centered().white().on_black(),
                layout[0],
            );
            f.render_stateful_widget(
                List::new(
                    tasks
                        .iter()
                        .map(|t| t.name.clone())
                        .collect::<Vec<String>>(),
                )
                .block(Block::bordered().title("Tasks"))
                .highlight_style(Style::new().red().italic())
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::WhenSelected),
                layout[1],
                &mut list_state,
            );
            f.render_widget(
                Paragraph::new("Shows possible commands here")
                    .centered()
                    .white()
                    .on_black(),
                layout[2],
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Up => list_state
                        .select(max(Some(0), Some(list_state.selected().unwrap_or(0) - 1))),
                    event::KeyCode::Down => list_state.select(min(
                        Some(tasks.len() - 1),
                        Some(list_state.selected().unwrap_or(0) + 1),
                    )),
                    _ => {}
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok("Exiting TUI...".to_string())
}

// Custom widget to list the tasks
