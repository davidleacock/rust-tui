use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Color, Modifier, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use crate::app::App;

pub fn draw_ui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame| {
        let size = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(70)
                ]
                    .as_ref(),
            )
            .split(size);

        let items: Vec<ListItem> = app.items.iter().enumerate().map(|(i, item)| {
            let status = if item.checked { "✅" } else { "⭕️" };

            if i == app.selected && app.editing {
                ListItem::new(format!("{} - {}", status, app.current_input.clone()))
                    .style(
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    )
            } else if i == app.selected {
                ListItem::new(format!("{} - {}", status, item.description))
                    .style(
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    )
            } else {
                ListItem::new(format!("{} - {}", status, item.description))
            }
        }).collect();

        // TODO: Change boarder color/style on highlighting
        let todo_list =
            List::new(items).block(Block::default().title("ToDo List").borders(Borders::ALL));

        frame.render_widget(todo_list, chunks[0]);

        let notepad = Paragraph::new(app.notepad_content.clone())
            .block(Block::default().title("Notepad").borders(Borders::ALL));


        frame.render_widget(notepad, chunks[1]);
    })?;

    Ok(())
}