use crate::commands::db::DB;
use crate::commands::tui::ui::ui;
use ratatui::backend::Backend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::Terminal;

pub(super) enum CurrentScreen {
    TaskList,
    Task,
}

pub(super) struct App<'a> {
    pub(super) current_screen: CurrentScreen,
    db: &'a DB,
}

impl<'a> App<'a> {
    pub(super) fn new(db: &'a DB) -> Self {
        App {
            current_screen: CurrentScreen::TaskList,
            db,
        }
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
                        _ => (),
                    },
                    CurrentScreen::Task => todo!("Not implemented yet"),
                }
            }
        }
        Ok(())
    }
}
