use ratatui::prelude::{Color, Modifier, Style};
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, List, ListItem};
use crate::app::TodoApp;

pub fn draw_ui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &TodoApp) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame| {
        let size = frame.area();

        let items: Vec<ListItem> = app.items.iter().enumerate().map(|(i, item)| {
            let status = if item.checked { "✅" } else { "⭕️" };

            if i == app.selected && app.editing {
                ListItem::new(format!("{} - {}", status, app.current_input.clone()))
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else if i == app.selected {
                ListItem::new(format!("{} - {}", status, item.description))
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(format!("{} - {}", status, item.description))
            }
        }).collect();

        let list = List::new(items).block(Block::default().title("ToDo List").borders(Borders::ALL));

        frame.render_widget(list, size);
    })?;

    Ok(())
}