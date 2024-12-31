use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::list;
use crate::commands::model::Task;
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::*;
use ratatui::widgets::{Block, List};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    CompletedFrame, Terminal,
};
use std::error::Error;
use std::io;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub db: &'a DB,
}

impl<'a> App<'a> {
    pub fn new(db: &'a DB) -> Self {
        App {
            current_screen: CurrentScreen::Main,
            db,
        }
    }

    fn render_main<'b>(
        tasks: &Result<CommandResult<Vec<Task>>, Box<dyn Error>>,
        terminal: &'b mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> io::Result<CompletedFrame<'b>> {
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(20),
                    Constraint::Percentage(70),
                    Constraint::Percentage(10),
                ])
                .split(frame.area());

            let title = Paragraph::new("Todo Manager".bold().to_string())
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::Blue));
            frame.render_widget(title, layout[0]);

            match tasks {
                Ok(cmd_result) => {
                    let tasks = cmd_result.result();
                    let list = List::new(tasks.iter().map(|t| t.name.as_str()))
                        .block(Block::default().borders(ratatui::widgets::Borders::ALL));
                    frame.render_widget(list, layout[1]);
                }
                Err(e) => {
                    // TODO: add better error handling
                    frame.render_widget(Text::from(e.to_string()), layout[1]);
                }
            }

            let help_message = Paragraph::new("Press 'q' to quit");
            frame.render_widget(help_message, layout[2]);
        })
    }

    pub async fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        let tasks: Result<CommandResult<Vec<Task>>, Box<dyn Error>> =
            list::run(self.db, false).await;
        loop {
            match self.current_screen {
                CurrentScreen::Main => {
                    Self::render_main(&tasks, &mut terminal).expect("TODO: panic message");
                }
                CurrentScreen::Editing => {
                    // let _ = Self::render_main(&mut terminal);
                    todo!()
                }
                CurrentScreen::Exiting => {
                    break;
                }
            }

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('e') {
                    self.current_screen = CurrentScreen::Editing;
                }
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.current_screen = CurrentScreen::Exiting;
                }
            }
        }

        Ok(())
    }
}

pub(crate) async fn run(db: &DB) -> Result<CommandResult<()>, Box<dyn std::error::Error>> {
    // start terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new(db);
    app.run(terminal).await?;

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen,)?;

    Ok(CommandResult::new("".to_string(), ()))
}
