mod app;
mod ui;

use app::App;

use crate::cli::model::CommandResult;
use crate::commands::db::DB;
use ratatui::{
    crossterm::{
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    Terminal,
};
use std::error::Error;
use std::io;

// TODO: clean up this file when done with refactor
//
// const KEYBIND_COUNT: usize = 5;
// const KEYBIND_QUIT: KeyCode = KeyCode::Char('q');
// const KEYBIND_EDIT: KeyCode = KeyCode::Char('e');
// const KEYBIND_UP: KeyCode = KeyCode::Up;
// const KEYBIND_DOWN: KeyCode = KeyCode::Down;
// const KEYBIND_ENTER: KeyCode = KeyCode::Enter;
//
// enum CurrentScreen {
//     Main,
//     Exiting,
//     Viewing,
//     Input,
// }
//
// struct App<'a> {
//     current_screen: CurrentScreen,
//     db: &'a DB,
//     list_state: ListState,
//     edit_state: TableState,
//     is_editing: bool,
//     tasks: Vec<Task>,
// }
//
// impl<'a> App<'a> {
//     fn new(db: &'a DB) -> Self {
//         App {
//             current_screen: CurrentScreen::Main,
//             db,
//             list_state: ListState::default(),
//             edit_state: TableState::default(),
//             is_editing: false,
//             tasks: vec![],
//         }
//     }
//
//     async fn update_tasks(&mut self) {
//         let tasks: Result<CommandResult<Vec<Task>>, Box<dyn Error>> =
//             list::run(self.db, false).await;
//         match tasks {
//             Ok(cmd_result) => {
//                 self.tasks = cmd_result.result().to_vec();
//                 if !self.tasks.is_empty() {
//                     self.list_state.select(Some(0));
//                 }
//             }
//             Err(e) => {
//                 eprintln!("Error getting tasks: {}", e);
//             }
//         }
//     }
//
//     fn get_tasks(&self) -> &Vec<Task> {
//         &self.tasks
//     }
//
//     fn widget_title() -> Paragraph<'a> {
//         Paragraph::new("Todo Manager".bold().to_string())
//             .alignment(Alignment::Center)
//             .style(Style::default().fg(Color::Blue))
//     }
//
//     fn widget_list(tasks: &[Task]) -> List {
//         List::new(tasks.iter().map(|t| t.name.to_string()))
//             .block(Block::default().borders(ratatui::widgets::Borders::ALL))
//             .highlight_symbol("> ")
//     }
//
//     fn widget_list_task(task: &Task, is_editing: bool) -> Table {
//         let rows = vec![
//             Row::new(vec![
//                 "id".to_string(),
//                 task.get_id().expect("Failed to get id").to_string(),
//             ]),
//             Row::new(vec!["name".to_string(), task.name.to_string()]),
//             Row::new(vec!["priority".to_string(), task.priority.to_string()]),
//             Row::new(vec![
//                 "description".to_string(),
//                 task.description
//                     .as_ref()
//                     .unwrap_or(&"".to_string())
//                     .to_string(),
//             ]),
//             Row::new(vec![
//                 "work note path".to_string(),
//                 task.work_note_path
//                     .as_ref()
//                     .unwrap_or(&"".to_string())
//                     .to_string(),
//             ]),
//             Row::new(vec!["created at".to_string(), task.created_at.to_string()]),
//             Row::new(vec![
//                 "completed at".to_string(),
//                 task.completed_at
//                     .clone()
//                     .map(|s| s.to_string())
//                     .unwrap_or("In progress".to_string()),
//             ]),
//         ];
//         let mut table = Table::new(
//             rows,
//             [Constraint::Percentage(50), Constraint::Percentage(50)],
//         );
//         if is_editing {
//             table = table.highlight_symbol("> ");
//         }
//         table
//     }
//
//     fn widget_keybinds() -> Table<'a> {
//         // Renders like:  Key | Description
//         assert_eq!(KEYBIND_COUNT, 5);
//         let rows = vec![
//             Row::new(vec!["↑", "Previous Task"]),
//             Row::new(vec!["↓", "Next Task"]),
//             Row::new(vec!["Enter", "Select Task"]),
//             Row::new(vec!["e", "Edit Current Task"]),
//             Row::new(vec!["q", "Quit"]),
//         ];
//         Table::new(
//             rows,
//             [Constraint::Percentage(10), Constraint::Percentage(90)],
//         )
//         .header(
//             Row::new(vec!["Key", "Action"])
//                 .bottom_margin(1)
//                 .style(Style::default().add_modifier(Modifier::BOLD)),
//         )
//         .column_spacing(1)
//         .block(Block::default())
//     }
//
//     fn app_layout() -> Layout {
//         Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(vec![
//                 Constraint::Percentage(5),
//                 Constraint::Percentage(80),
//                 Constraint::Percentage(15),
//             ])
//     }
//
//     async fn run(
//         &mut self,
//         mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
//     ) -> io::Result<()> {
//         self.update_tasks().await;
//
//         loop {
//             match self.current_screen {
//                 CurrentScreen::Main => {
//                     let title_widget = Self::widget_title();
//                     let list_widget = Self::widget_list(self.get_tasks());
//                     let keybind_widget = Self::widget_keybinds();
//                     let mut list_state = self.list_state.clone();
//
//                     terminal
//                         .draw(|frame| {
//                             let layout = Self::app_layout().split(frame.area());
//                             frame.render_widget(title_widget, layout[0]);
//                             frame.render_stateful_widget(list_widget, layout[1], &mut list_state);
//                             frame.render_widget(keybind_widget, layout[2]);
//                         })
//                         .expect("Could not render main screen");
//
//                     // TODO: there's probably a better way
//                     if let event::Event::Key(key) = event::read()? {
//                         assert_eq!(KEYBIND_COUNT, 5);
//                         if key.kind == KeyEventKind::Press {
//                             if key.code == KEYBIND_QUIT {
//                                 self.current_screen = CurrentScreen::Exiting;
//                             }
//                             if key.code == KEYBIND_EDIT {}
//                             if key.code == KEYBIND_ENTER {
//                                 self.current_screen = CurrentScreen::Viewing;
//                             }
//                             if key.code == KEYBIND_UP {
//                                 list_state.select_previous();
//                             }
//                             if key.code == KEYBIND_DOWN {
//                                 list_state.select_next();
//                             }
//                         }
//                     }
//
//                     self.list_state = list_state;
//                 }
//                 CurrentScreen::Viewing => {
//                     let current_task = self.list_state.selected().expect("No task selected");
//                     // TODO: Layout split horizontal where left is task info and right is work note
//                     let title_widget = Self::widget_title();
//                     let list_widget =
//                         Self::widget_list_task(&self.tasks[current_task], self.is_editing);
//                     let keybind_widget = Self::widget_keybinds();
//                     let mut edit_state = self.edit_state.clone();
//
//                     terminal
//                         .draw(|frame| {
//                             let layout = Self::app_layout().split(frame.area());
//                             frame.render_widget(title_widget, layout[0]);
//                             frame.render_stateful_widget(list_widget, layout[1], &mut edit_state);
//                             frame.render_widget(keybind_widget, layout[2]);
//                         })
//                         .expect("Could not render main screen");
//
//                     // TODO: there's probably a better way
//                     if let event::Event::Key(key) = event::read()? {
//                         assert_eq!(KEYBIND_COUNT, 5);
//                         if key.kind == KeyEventKind::Press {
//                             if key.code == KEYBIND_QUIT && !self.is_editing {
//                                 self.current_screen = CurrentScreen::Exiting;
//                             }
//                             if key.code == KEYBIND_EDIT {
//                                 self.is_editing = !self.is_editing;
//                                 edit_state.select_first();
//                             }
//                             if key.code == KEYBIND_ENTER && self.is_editing {
//                                 // TODO: this should update task with typable field
//                                 self.current_screen = CurrentScreen::Input;
//                             }
//                             if key.code == KEYBIND_UP && self.is_editing {
//                                 edit_state.select_previous();
//                             }
//                             if key.code == KEYBIND_DOWN && self.is_editing {
//                                 edit_state.select_next();
//                             }
//                         }
//                     }
//
//                     self.edit_state = edit_state;
//                 }
//                 CurrentScreen::Input => {
//                     let title_widget = Self::widget_title();
//                     // TODO: not the biggest fan of this, find better way
//                     let fields = vec![
//                         "id",
//                         "name",
//                         "priority",
//                         "description",
//                         "work note path",
//                         "created at",
//                         "completed at",
//                     ];
//                     let field = self.edit_state.selected().expect("No field selected");
//                     let curr_task =
//                         self.tasks[self.list_state.selected().expect("No task selected")].clone();
//                     let input_widget = match fields[field] {
//                         "name" => InputBox::new(
//                             self.tasks[self.edit_state.selected().unwrap()]
//                                 .name
//                                 .to_string(),
//                         ),
//                         "priority" => InputBox::new(
//                             self.tasks[self.edit_state.selected().unwrap()]
//                                 .priority
//                                 .to_string(),
//                         ),
//                         "description" => {
//                             todo!()
//                             // InputBox::new(self.tasks[self.edit_state.selected().unwrap()].description.unwrap_or("".parse().unwrap()).to_string())
//                         }
//                         _ => InputBox::new("Unable to update this field".to_string()),
//                     };
//                     let input_widget = InputBox::new(
//                         self.tasks[self.edit_state.selected().unwrap()]
//                             .name
//                             .to_string(),
//                     );
//                     let keybind_widget = Self::widget_keybinds();
//                 }
//                 CurrentScreen::Exiting => {
//                     break;
//                 }
//             }
//         }
//
//         Ok(())
//     }
// }

pub(crate) async fn run(db: &DB) -> Result<CommandResult<()>, Box<dyn Error>> {
    // start terminal
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let mut app = App::new(db);
    let _res = app.run(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen,)?;

    Ok(CommandResult::new("".to_string(), ()))
}
