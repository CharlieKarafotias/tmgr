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
use std::io::{stdout, Stdout};

pub(crate) async fn run(db: &DB) -> Result<String, Box<dyn std::error::Error>> {
    println!("Starting TUI...");
    let mut terminal = terminal_setup()?;
    let mut state = AppState::new(db).await;

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
                    state
                        .tasks
                        .iter()
                        .map(|t| t.name.as_str())
                        .collect::<Vec<&str>>(),
                )
                .block(Block::bordered().title("Tasks"))
                .highlight_style(Style::new().red().italic())
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::WhenSelected),
                layout[1],
                &mut state.list_state,
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
                    event::KeyCode::Up => state.list_state.select_previous(),
                    event::KeyCode::Down => state.list_state.select_next(),
                    _ => {}
                }
            }
        }
    }

    terminal_cleanup()?;
    Ok("Exiting TUI...".to_string())
}

fn terminal_setup() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

fn terminal_cleanup() -> Result<(), Box<dyn std::error::Error>> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

struct AppState {
    list_state: ListState,
    tasks: Vec<Task>,
}

impl AppState {
    async fn new(db: &DB) -> Self {
        let tasks = db.client.select("task").await.unwrap_or_default();
        let mut list_state = ListState::default();
        if !tasks.is_empty() {
            list_state.select(Some(0));
        }
        Self { list_state, tasks }
    }
}
