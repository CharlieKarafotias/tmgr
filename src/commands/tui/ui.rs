use crate::commands::{
    model::Task,
    tui::app::{App, CurrentScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table},
    Frame,
};

pub(super) fn ui(frame: &mut Frame, app: &mut App) {
    match app.get_current_screen() {
        CurrentScreen::Main => {
            let block = outer_block();
            let inner_area = block.inner(frame.area());
            let layout = layout(vec![90, 10], Direction::Vertical);
            let l = layout.split(inner_area);

            frame.render_widget(block, frame.area());
            // TODO: update to table stateful widget
            frame.render_stateful_widget(table_widget(&app.tasks), l[0], &mut app.table_state);
            frame.render_widget(keybind_widget(app, app.get_current_screen()), l[1]);
        }
        CurrentScreen::Task => {
            todo!("Task edit screen");
            // let layout = layout(vec![5, 85, 10], Direction::Vertical);
            // let l = layout.split(frame.area());
            // let current_task = &app.tasks[app.list_state.selected().expect("No task selected")];
            // frame.render_widget(title_widget(), l[0]);
            // // TODO: add ability to update fields of Task
            // frame.render_widget(edit_widget(current_task), l[1]);
            // frame.render_widget(keybind_widget(app, app.get_current_screen()), l[2]);
        }
        _ => {}
    }
}

// --- Widgets ---

/// Constructs a `Block` widget used as the outermost border
/// for all screens in the UI.
///
/// This block is configured to have a centered title with the text
/// `Todo Task Manager`, and to have a white, all-around border.
///
/// # Returns
///
/// The constructed `Block` widget.
fn outer_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White))
        .title("Todo Task Manager".to_owned())
        .title_alignment(Alignment::Center)
}

fn table_widget(tasks: &[Task]) -> Table {
    Table::default()
        .header(Row::new(vec!["ID", "Name", "Created At"]))
        .rows(tasks.iter().map(|t| {
            Row::new(vec![
                t.get_id().unwrap_or_default(),
                t.name.clone(),
                t.created_at.format("%D").to_string(),
            ])
        }))
        .highlight_symbol("> ")
}

/// Constructs a `Paragraph` widget displaying keybindings for the UI.
///
/// The widget comprises a centered list of key-action pairs, indicating
/// the available keyboard shortcuts and their respective actions
/// within the application. The keybindings are presented in a format
/// where each key is followed by its description, separated by " - ",
/// and each pair is joined by " | " for display. The paragraph is
/// styled with top and bottom borders and a centered title "Keybinds".
///
/// Returns:
///     A `Paragraph` widget configured with the keybindings display.
fn keybind_widget(app: &App, current_screen: &CurrentScreen) -> Paragraph<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title_top("Keybinds")
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));

    let bindings = app
        .get_keybindings(current_screen)
        .unwrap_or(&vec![])
        .iter()
        .map(|k| format!("[{}] {}", k.key(), k.description()))
        .collect::<Vec<String>>()
        .join("  ");

    Paragraph::new(bindings)
        .centered()
        .block(block)
        .style(Style::default().fg(Color::Blue))
}

fn edit_widget(task: &Task) -> Table {
    let constraints = Constraint::from_percentages([25, 75]);
    let rows: Vec<Row> = task
        .key_values()
        .iter()
        .map(|(k, v)| Row::new(vec![k.to_string(), v.to_string()]))
        .collect();
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Viewing task ".to_owned() + task.get_id().unwrap_or_default().as_str())
        .title_alignment(Alignment::Center)
        .padding(Padding::uniform(1));
    Table::new(rows, constraints)
        .block(block)
        .header(Row::new(vec!["Key", "Value"]))
}

/// Construct a Layout with the given constraints and direction.
///
/// # Arguments
///
/// * `constraints`: A vector of percentages that represent the width of each
///   item in the layout.
/// * `direction`: The direction of the layout.
///
/// # Returns
///
/// A Layout with the given constraints and direction.
fn layout(constraints: Vec<u16>, direction: Direction) -> Layout {
    let constraints: Vec<Constraint> = constraints
        .iter()
        .map(|c| Constraint::Percentage(*c))
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
}
