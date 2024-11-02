use crate::app::{App, TodoItem, Window};
use chrono::Local;
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

        let total_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
            .split(size);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(total_chunks[0]);

        let todo_items: Vec<ListItem> = if app.todo_items.is_empty() {
            vec![
                ListItem::new("No current tasks. Press `I` to insert.").style(
                    Style::default()
                        .fg(Color::Blue)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]
        } else {
            app.todo_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let status = if item.checked { "✅" } else { "⭕️" };

                    if i == app.current_todo_item && app.todo_editing_mode {
                        ListItem::new(format!("{} - {}", status, app.current_todo_input.clone()))
                            .style(
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            )
                    } else if i == app.current_todo_item {
                        ListItem::new(format!("{} - {}", status, item.description)).style(
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ListItem::new(format!("{} - {}", status, item.description))
                    }
                })
                .collect()
        };

        let todo_list = List::new(todo_items)
            .block(styled_block("Todo", app.active_window == Window::TodoList));

        let notepad = Paragraph::new(app.notepad_content.clone()).block(styled_block(
            "Notepad",
            app.active_window == Window::Notepad,
        ));

        let footer = render_footer(app);

        frame.render_widget(todo_list, content_chunks[0]);
        frame.render_widget(notepad, content_chunks[1]);
        frame.render_widget(footer, total_chunks[1])
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

fn render_footer(app: &App) -> Paragraph<'static> {
    let now = Local::now();
    let datetime = now.format("%Y-%m-%d %H:%M:%S").to_string();

    let help_text = match app.active_window {
        Window::TodoList => "Keys: [E] Edit  [I] Insert  [R] Remove  [Space] Toggle Check",
        Window::Notepad => "Keys: [E] Edit Note  [Enter] New Line",
    };

    let footer_text = format!("{} | {}", help_text, datetime);

    Paragraph::new(footer_text)
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title("Info"),
        )
}
