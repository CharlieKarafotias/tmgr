use crate::commands::{
    model::Task,
    tui::app::{App, CurrentScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Padding, Paragraph, Row, Table, Wrap},
    Frame,
};

// TODO: move these out into own components instead of defining everything here
pub(super) fn ui(frame: &mut Frame, app: &mut App) {
    match app.get_current_screen() {
        CurrentScreen::Main => {
            let block = outer_block();
            let inner_area = block.inner(frame.area());
            let layout = layout(vec![15, 65, 20], Direction::Vertical);
            let l = layout.split(inner_area);

            frame.render_widget(block, frame.area());
            frame.render_stateful_widget(table_widget(&app.tasks), l[0], &mut app.table_state);
            frame.render_widget(
                task_details_widget(
                    &app.tasks[app.table_state.selected().expect("No task selected")],
                ),
                l[1],
            );
            frame.render_widget(keybind_widget(app, app.get_current_screen()), l[2]);
        }
        CurrentScreen::Task => {
            let task_id = &app.tasks[app.table_state.selected().unwrap_or_default()];
            let block = Block::bordered()
                .title(format!(
                    "Editing task {}",
                    task_id.get_id().unwrap_or_default()
                ))
                .title_alignment(Alignment::Center);
            let table = popup_table(task_id).block(block);
            let area = popup_area(frame.area(), 50, 50);
            frame.render_widget(table, area);
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

fn task_details_widget(task: &Task) -> Paragraph<'static> {
    let s = format!("Task Details:\n{}", task).replace("\n", "\n\n");
    Paragraph::new(s)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .padding(Padding::left(2)),
        )
        .wrap(Wrap { trim: true })
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
        .wrap(Wrap { trim: true })
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn popup_table(task: &Task) -> Table {
    Table::default()
        .header(Row::new(vec!["Key", "Value"]))
        .rows(vec![
            Row::new(vec!["Name", &task.name]),
            Row::new(vec!["Priority", &task.priority]),
            Row::new(vec![
                "Description",
                task.description.as_deref().unwrap_or(""),
            ]),
        ])
        .highlight_symbol("> ")
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
