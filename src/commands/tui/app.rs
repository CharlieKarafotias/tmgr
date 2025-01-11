use crate::commands::{db::DB, delete, list, model::Task, tui::ui::ui};
use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode},
    widgets::TableState,
    Terminal,
};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub(super) enum CurrentScreen {
    Main,
    Task,
    Exit,
}

pub(super) struct App<'a> {
    current_screen: CurrentScreen,
    db: &'a DB,
    keybindings: HashMap<CurrentScreen, Vec<KeyBinding>>,
    pub(super) table_state: TableState,
    pub(super) tasks: Vec<Task>,
    should_delete: bool, // TODO: turn to actions queue instead that executes all DB actions at end of draw loop
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

impl<'a> App<'a> {
    pub(super) async fn new(db: &'a DB) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: could list take in a filter & sort param too?
        let list_cmd_res = list::run(db, false).await?;
        // TODO: is there a better way than cloning?
        let tasks = list_cmd_res.result().clone();
        let mut table_state = TableState::default();
        table_state.select_first();

        let keybindings = HashMap::from([
            (
                CurrentScreen::Main,
                vec![
                    KeyBinding::new(KeyCode::Char('a'), String::from("Add Task"), |app| {
                        app.current_screen = CurrentScreen::Task
                    }),
                    KeyBinding::new(KeyCode::Char('e'), String::from("Edit Task"), |app| {
                        app.current_screen = CurrentScreen::Task
                    }),
                    KeyBinding::new(KeyCode::Char('d'), String::from("Delete Task"), |app| {
                        app.should_delete = true
                    }),
                    KeyBinding::new(KeyCode::Char('q'), String::from("Quit"), |app| {
                        app.current_screen = CurrentScreen::Exit
                    }),
                    KeyBinding::new(KeyCode::Up, String::from("Previous Task"), |app| {
                        app.table_state.select_previous()
                    }),
                    KeyBinding::new(KeyCode::Down, String::from("Next Task"), |app| {
                        app.table_state.select_next()
                    }),
                ],
            ),
            (
                CurrentScreen::Task,
                vec![KeyBinding::new(
                    KeyCode::Char('q'),
                    String::from("Quit"),
                    |app| app.current_screen = CurrentScreen::Main,
                )],
            ),
        ]);

        Ok(App {
            current_screen: CurrentScreen::Main,
            db,
            keybindings,
            table_state,
            tasks,
            should_delete: false,
        })
    }

    pub(super) async fn run<B: Backend>(
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
            if self.should_delete {
                self.delete_task().await?;
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

    async fn delete_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let task_idx = self.table_state.selected().ok_or("No task selected")?;
        let task_id = self.tasks[task_idx].get_id()?;
        delete::run(self.db, task_id).await?;
        // TODO: after live query implemented, no need to do this
        self.tasks.remove(task_idx);
        self.should_delete = false;
        Ok(())
    }
}
