use crate::commands::db::DB;
use crate::commands::model::Task;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::{Buffer, Color, Constraint, Direction, Layout, Line, Rect, Style, Widget},
    style::Stylize,
    text::Span,
    widgets::{Block, HighlightSpacing, List, ListState, Paragraph},
    Terminal,
};
use std::io::{stdout, Stdout};

pub(crate) async fn run(db: &DB) -> Result<String, Box<dyn std::error::Error>> {
    println!("Starting TUI...");
    let mut terminal = terminal_setup()?;
    let style = Style::new().fg(Color::White).bg(Color::Black);
    let mut state = AppState::new(db).await;

    loop {
        terminal.draw(|f| {
            // TODO: factor out into function
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ])
                .split(f.area());
            render_top_bar(layout[0], f.buffer_mut(), style);
            // TODO: factor out into function
            f.render_stateful_widget(
                List::new(
                    state
                        .tasks
                        .iter()
                        .map(|t| t.name.as_str())
                        .collect::<Vec<&str>>(),
                )
                .block(Block::bordered().title("Tasks"))
                .style(style)
                .highlight_style(Style::new().red().italic())
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::WhenSelected),
                layout[1],
                &mut state.list_state,
            );
            render_bottom_bar(layout[2], f.buffer_mut());
            // TODO: RENDER overlay here (use app state variable to trigger this to happen)
            // on enter set the overlay to true
            // Then use Clear widget to clear area
            // Then render the overlay
            // Should show the details of the current task + options to edit
            // on escape set the overlay to false
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    event::KeyCode::Char('q') => break,
                    event::KeyCode::Up => state.list_state.select_previous(),
                    event::KeyCode::Down => state.list_state.select_next(),
                    event::KeyCode::Enter => todo!("Implement select task"),
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

fn render_top_bar(area: Rect, buf: &mut Buffer, style: Style) {
    Paragraph::new("Task Manager (TMGR)")
        .centered()
        .style(style)
        .render(area, buf);
}
fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
    let keys = [
        ("q", "Quit"),
        ("↑", "Previous"),
        ("↓", "Next"),
        ("Enter", "Select"),
    ];
    let spans: Vec<Span> = keys
        .iter()
        .flat_map(|(key, desc)| {
            let key = Span::styled(
                format!(" {key} "),
                Style::new().fg(Color::Red).bg(Color::Black),
            );
            let desc = Span::styled(
                format!("- {desc} "),
                Style::new().fg(Color::White).bg(Color::Black),
            );
            [key, desc]
        })
        .collect();
    Line::from(spans)
        .centered()
        .style((Color::Indexed(236), Color::Indexed(232)))
        .render(area, buf);
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
