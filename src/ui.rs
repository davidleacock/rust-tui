use crate::app::{App, Window};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

pub fn draw_ui<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &App,
) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame| {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(size);

        let todo_items: Vec<ListItem> = app
            .items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let status = if item.checked { "✅" } else { "⭕️" };

                if i == app.selected && app.editing {
                    ListItem::new(format!("{} - {}", status, app.current_input.clone())).style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else if i == app.selected {
                    ListItem::new(format!("{} - {}", status, item.description)).style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(format!("{} - {}", status, item.description))
                }
            })
            .collect();

        let todo_list = List::new(todo_items)
            .block(styled_block("Todo", app.active_window == Window::TodoList));

        let notepad = Paragraph::new(app.notepad_content.clone()).block(styled_block(
            "Notepad",
            app.active_window == Window::Notepad,
        ));

        frame.render_widget(todo_list, chunks[0]);
        frame.render_widget(notepad, chunks[1]);
    })?;

    Ok(())
}

fn styled_block(title: &'static str, is_active: bool) -> Block<'static> {
    if is_active {
        Block::default()
            .border_type(BorderType::Thick)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Black))
            .title(title)
    } else {
        Block::default().title(title).borders(Borders::ALL)
    }
}
