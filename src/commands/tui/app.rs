use crate::commands::db::DB;
use crate::commands::list;
use crate::commands::model::Task;
use crate::commands::tui::ui::ui;
use ratatui::backend::Backend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::widgets::ListState;
use ratatui::Terminal;

pub(super) enum CurrentScreen {
    TaskList,
    Task,
}

pub(super) struct App {
    pub(super) current_screen: CurrentScreen,
    pub(super) list_state: ListState,
    pub(super) tasks: Vec<Task>,
}

impl App {
    pub(super) async fn new(db: &DB) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: could list take in a filter & sort param too?
        let list_cmd_res = list::run(db, false).await?;
        // TODO: is there a better way than cloning?
        let tasks = list_cmd_res.result().clone();
        let mut list_state = ListState::default();
        list_state.select_first();

        Ok(App {
            current_screen: CurrentScreen::TaskList,
            list_state,
            tasks,
        })
    }

    pub(super) fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            terminal.draw(|f| ui(f, self))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    continue;
                }

                match self.current_screen {
                    CurrentScreen::TaskList => match key.code {
                        event::KeyCode::Char('e') => {
                            self.current_screen = CurrentScreen::Task;
                        }
                        event::KeyCode::Char('q') => {
                            break;
                        }
                        event::KeyCode::Up => self.list_state.select_previous(),
                        event::KeyCode::Down => self.list_state.select_next(),
                        _ => (),
                    },
                    CurrentScreen::Task => todo!("Not implemented yet"),
                }
            }
        }
        Ok(())
    }
}
