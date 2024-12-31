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
    widgets::{Block, List, ListState, Paragraph, Row, Table},
    Terminal,
};
use std::error::Error;
use std::io;

const KEYBIND_COUNT: usize = 4;
const KEYBIND_QUIT: KeyCode = KeyCode::Char('q');
const KEYBIND_EDIT: KeyCode = KeyCode::Char('e');
const KEYBIND_UP: KeyCode = KeyCode::Up;
const KEYBIND_DOWN: KeyCode = KeyCode::Down;

enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

struct App<'a> {
    current_screen: CurrentScreen,
    db: &'a DB,
    list_state: ListState,
    tasks: Vec<Task>,
}

impl<'a> App<'a> {
    fn new(db: &'a DB) -> Self {
        App {
            current_screen: CurrentScreen::Main,
            db,
            list_state: ListState::default(),
            tasks: vec![],
        }
    }

    async fn update_tasks(&mut self) {
        let tasks: Result<CommandResult<Vec<Task>>, Box<dyn Error>> =
            list::run(self.db, false).await;
        match tasks {
            Ok(cmd_result) => {
                self.tasks = cmd_result.result().to_vec();
                if !self.tasks.is_empty() {
                    self.list_state.select(Some(0));
                }
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

    fn widget_keybinds() -> Table<'a> {
        // Renders like:  Key | Description
        assert_eq!(KEYBIND_COUNT, 4);
        let rows = vec![
            Row::new(vec!["↑", "Previous Task"]),
            Row::new(vec!["↓", "Next Task"]),
            Row::new(vec!["e", "Edit Current Task"]),
            Row::new(vec!["q", "Quit"]),
        ];
        Table::new(
            rows,
            [Constraint::Percentage(10), Constraint::Percentage(90)],
        )
        .header(
            Row::new(vec!["Key", "Action"])
                .bottom_margin(1)
                .style(Style::default().add_modifier(Modifier::BOLD)),
        )
        .column_spacing(1)
        .block(Block::default())
    }

    fn app_layout() -> Layout {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(80),
                Constraint::Percentage(15),
            ])
    }

    async fn run(
        &mut self,
        mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        self.update_tasks().await;

        loop {
            match self.current_screen {
                CurrentScreen::Main => {
                    let title_widget = Self::widget_title();
                    let list_widget = Self::widget_list(self.get_tasks());
                    let keybind_widget = Self::widget_keybinds();
                    let mut list_state = self.list_state.clone();

                    terminal
                        .draw(|frame| {
                            let layout = Self::app_layout().split(frame.area());
                            frame.render_widget(title_widget, layout[0]);
                            // TODO: update to stateful
                            frame.render_stateful_widget(list_widget, layout[1], &mut list_state);
                            frame.render_widget(keybind_widget, layout[2]);
                        })
                        .expect("Could not render main screen");

                    // TODO: there's probably a better way
                    if let event::Event::Key(key) = event::read()? {
                        assert_eq!(KEYBIND_COUNT, 4);
                        if key.kind == KeyEventKind::Press {
                            if key.code == KEYBIND_QUIT {
                                self.current_screen = CurrentScreen::Exiting;
                            }
                            if key.code == KEYBIND_EDIT {
                                self.current_screen = CurrentScreen::Editing;
                            }
                            if key.code == KEYBIND_UP {
                                list_state.select_previous();
                            }
                            if key.code == KEYBIND_DOWN {
                                list_state.select_next();
                            }
                        }
                    }

                    self.list_state = list_state;
                }
                CurrentScreen::Editing => {
                    todo!()
                }
                CurrentScreen::Exiting => {
                    break;
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
