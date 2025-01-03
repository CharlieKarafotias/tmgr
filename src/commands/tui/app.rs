use crate::commands::{db::DB, list, model::Task, tui::ui::ui};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    widgets::ListState,
    Terminal,
};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub(super) enum CurrentScreen {
    TaskList,
    Task,
}

pub(super) struct App {
    current_screen: CurrentScreen,
    keybindings: HashMap<CurrentScreen, Vec<KeyBinding>>,
    pub(super) list_state: ListState,
    pub(super) tasks: Vec<Task>,
}

pub(super) struct KeyBinding {
    key: KeyCode,
    description: String,
    action: fn(&mut App),
}

impl KeyBinding {
    fn new(key: KeyCode, description: String, action: fn(&mut App)) -> Self {
        KeyBinding {
            key,
            description,
            action,
        }
    }

    pub(super) fn key(&self) -> KeyCode {
        self.key
    }

    pub(super) fn description(&self) -> String {
        self.description.clone()
    }
}

impl App {
    pub(super) async fn new(db: &DB) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: could list take in a filter & sort param too?
        let list_cmd_res = list::run(db, false).await?;
        // TODO: is there a better way than cloning?
        let tasks = list_cmd_res.result().clone();
        let mut list_state = ListState::default();
        list_state.select_first();

        let keybindings = HashMap::from([
            (
                CurrentScreen::TaskList,
                vec![
                    KeyBinding::new(KeyCode::Enter, String::from("Select Task"), |app| {
                        app.set_current_screen(CurrentScreen::Task)
                    }),
                    KeyBinding::new(KeyCode::Char('q'), String::from("Quit"), |app| {
                        app.set_current_screen(CurrentScreen::TaskList)
                    }),
                    KeyBinding::new(KeyCode::Up, String::from("Previous Task"), |app| {
                        app.list_state.select_previous()
                    }),
                    KeyBinding::new(KeyCode::Down, String::from("Next Task"), |app| {
                        app.list_state.select_next()
                    }),
                ],
            ),
            (
                CurrentScreen::Task,
                vec![KeyBinding::new(
                    KeyCode::Char('q'),
                    String::from("Quit"),
                    |app| app.set_current_screen(CurrentScreen::TaskList),
                )],
            ),
        ]);

        Ok(App {
            current_screen: CurrentScreen::TaskList,
            keybindings,
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
                match key.kind {
                    // TODO: finish this, need to use keybindings and actions
                    event::KeyEventKind::Press => match self.current_screen {
                        CurrentScreen::TaskList => match key.code {
                            KeyCode::Enter => {
                                self.set_current_screen(CurrentScreen::Task);
                            }
                            KeyCode::Char('q') => {
                                break;
                            }
                            KeyCode::Up => self.list_state.select_previous(),
                            KeyCode::Down => self.list_state.select_next(),
                            _ => (),
                        },
                        CurrentScreen::Task => match key.code {
                            KeyCode::Char('q') => {
                                break;
                            }
                            _ => (),
                        },
                    },
                    _ => continue,
                }
            }
        }
        Ok(())
    }

    fn set_current_screen(&mut self, screen: CurrentScreen) {
        self.current_screen = screen;
        // TODO: update key bindings
    }

    pub(super) fn get_current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub(super) fn get_keybindings(&self, screen: &CurrentScreen) -> Option<&Vec<KeyBinding>> {
        self.keybindings.get(screen)
    }
}
