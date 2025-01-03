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
    Exit,
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
                        app.current_screen = CurrentScreen::Task
                    }),
                    KeyBinding::new(KeyCode::Char('q'), String::from("Quit"), |app| {
                        app.current_screen = CurrentScreen::Exit
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
                    |app| app.current_screen = CurrentScreen::TaskList,
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
            if self.current_screen == CurrentScreen::Exit {
                break;
            }
            terminal.draw(|f| ui(f, self))?;
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    self.get_keybind_action(&self.current_screen, &key.code)
                        .map_or((), |action| action(self))
                }
            }
        }
        Ok(())
    }

    pub(super) fn get_current_screen(&self) -> &CurrentScreen {
        &self.current_screen
    }

    pub(super) fn get_keybindings(&self, screen: &CurrentScreen) -> Option<&Vec<KeyBinding>> {
        self.keybindings.get(screen)
    }

    fn get_keybind_action(&self, screen: &CurrentScreen, key: &KeyCode) -> Option<fn(&mut App)> {
        self.keybindings
            .get(screen)
            .and_then(|keybindings| keybindings.iter().find(|kb| kb.key() == *key))
            .map(|kb| kb.action)
    }
}
