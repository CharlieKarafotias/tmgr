use crate::commands::{
    model::Task,
    tui::app::{App, CurrentScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, Paragraph},
    Frame,
};

pub(super) fn ui(frame: &mut Frame, app: &mut App) {
    match app.get_current_screen() {
        CurrentScreen::TaskList => {
            let layout = layout(vec![5, 85, 10], Direction::Vertical);
            let l = layout.split(frame.area());
            frame.render_widget(title_widget(), l[0]);
            frame.render_stateful_widget(list_widget(&app.tasks), l[1], &mut app.list_state);
            frame.render_widget(keybind_widget(app, app.get_current_screen()), l[2]);
        }
        CurrentScreen::Task => {
            let layout = layout(vec![5, 85, 10], Direction::Vertical);
            let l = layout.split(frame.area());
            frame.render_widget(title_widget(), l[0]);
            frame.render_stateful_widget(list_widget(&app.tasks), l[1], &mut app.list_state);
            frame.render_widget(keybind_widget(app, app.get_current_screen()), l[2]);
        }
        _ => {}
    }
}

// --- Widgets ---

/// A centered, blue, bold widget displaying the text `Todo Manager`.
///
/// This is used as the title widget in the main UI.
fn title_widget() -> Paragraph<'static> {
    Paragraph::new("Todo Manager".bold())
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Blue))
}

/// Constructs a `List` widget displaying a list of tasks.
///
/// The widget is configured to highlight the currently selected item
/// with a `"> "` symbol. The list items are the `name`s of the tasks
/// passed in.
///
/// # Arguments
///
/// * `tasks`: The tasks to display in the list.
///
/// # Returns
///
/// A `List` widget displaying the list of tasks.
fn list_widget(tasks: &[Task]) -> List {
    List::new(tasks.iter().map(|t| t.name.to_string())).highlight_symbol("> ")
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
    // TODO: add padding (getting error right now when I add padding where the bindings disappear)
    let block = Block::new()
        .borders(Borders::TOP | Borders::BOTTOM)
        .title_top("Keybinds")
        .title_alignment(Alignment::Center);

    let bindings = app
        .get_keybindings(current_screen)
        .unwrap_or(&vec![])
        .iter()
        .map(|k| format!("{} - {}", k.key(), k.description()))
        .collect::<Vec<String>>()
        .join(" | ");

    Paragraph::new(bindings)
        .centered()
        .block(block)
        .style(Style::default().fg(Color::Blue))
}

// TODO: implement edit_widget

// TODO: implement exit_widget

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
