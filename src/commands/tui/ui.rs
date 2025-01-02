use crate::commands::tui::app::{App, CurrentScreen};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub(super) fn ui(frame: &mut Frame, app: &App) {
    match app.current_screen {
        CurrentScreen::TaskList => {
            let layout = layout(vec![5, 85, 10], Direction::Vertical);
            let l = layout.split(frame.area());
            frame.render_widget(title_widget(), l[0]);
            frame.render_widget(keybind_widget(), l[2]);
        }
        CurrentScreen::Task => todo!(),
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

// TODO: implement list_widget

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
fn keybind_widget() -> Paragraph<'static> {
    // TODO: add padding (getting error right now when I add padding where the bindings disappear)
    let block = Block::new()
        .borders(Borders::TOP | Borders::BOTTOM)
        .title_top("Keybinds")
        .title_alignment(Alignment::Center);

    let bindings = [
        ("↑", "Previous Task"),
        ("↓", "Next Task"),
        ("Enter", "Select Task"),
        ("e", "Edit Current Task"),
        ("q", "Quit"),
    ]
    .iter()
    .map(|(key, description)| format!("{} - {}", key, description))
    .collect::<Vec<String>>()
    .join(" | ");

    Paragraph::new(bindings)
        .centered()
        .block(block)
        .style(Style::default().fg(Color::Blue))
}

// TODO: implement edit_widget

// TODO: implement exit_widget

fn layout(constraints: Vec<u16>, direction: Direction) -> Layout {
    let constraints: Vec<Constraint> = constraints
        .iter()
        .map(|c| Constraint::Percentage(*c))
        .collect();
    Layout::default()
        .direction(direction)
        .constraints(constraints)
}
