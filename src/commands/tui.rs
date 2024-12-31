use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use crate::commands::list;
use crate::commands::model::Task;
use ratatui::{
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    style::Stylize,
    widgets::{Block, List, ListState, Paragraph},
    Terminal,
};
use std::error::Error;
use std::io;

const KEYBIND_QUIT: char = 'q';
const KEYBIND_EDIT: char = 'e';

enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

struct App<'a> {
    current_screen: CurrentScreen,
    db: &'a DB,
    current_task: ListState,
    tasks: Vec<Task>,
}

impl<'a> App<'a> {
    fn new(db: &'a DB) -> Self {
        App {
            current_screen: CurrentScreen::Main,
            db,
            current_task: ListState::default(),
            tasks: vec![],
        }
    }

    async fn update_tasks(&mut self) {
        let tasks: Result<CommandResult<Vec<Task>>, Box<dyn Error>> =
            list::run(self.db, false).await;
        match tasks {
            Ok(cmd_result) => {
                self.tasks = cmd_result.result().to_vec();
            }
            Err(e) => {
                eprintln!("Error getting tasks: {}", e);
            }
        }
    }

    fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    fn widget_title() -> Paragraph<'a> {
        Paragraph::new("Todo Manager".bold().to_string())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Blue))
    }

    fn widget_list(tasks: &[Task]) -> List {
        List::new(tasks.iter().map(|t| t.name.to_string()))
            .block(Block::default().borders(ratatui::widgets::Borders::ALL))
            .highlight_symbol("> ")
    }

    async fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        self.update_tasks().await;

        loop {
            match self.current_screen {
                CurrentScreen::Main => {
                    let l = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(vec![
                            Constraint::Percentage(5),
                            Constraint::Percentage(90),
                            Constraint::Percentage(5),
                        ]);
                    let title_widget = Self::widget_title();
                    let list_widget = Self::widget_list(self.get_tasks());
                    let keybind_widget =
                        Paragraph::new("Press 'q' to quit").alignment(Alignment::Center);

                    terminal.draw(|frame| {
                        let layout = l.split(frame.area());
                        frame.render_widget(title_widget, layout[0]);
                        // TODO: update to stateful
                        frame.render_widget(list_widget, layout[1]);
                        frame.render_widget(keybind_widget, layout[2]);
                    })
                }
                CurrentScreen::Editing => {
                    // let _ = Self::render_main(&mut terminal);
                    todo!()
                }
                CurrentScreen::Exiting => {
                    break;
                }
            }
            .expect("Could not render terminal");

            // TODO: there's probably a better way
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char(KEYBIND_QUIT) {
                        self.current_screen = CurrentScreen::Exiting;
                    }
                    if key.code == KeyCode::Char(KEYBIND_EDIT) {
                        self.current_screen = CurrentScreen::Editing;
                    }
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
