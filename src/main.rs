use crossterm::event::{self, Event, KeyCode};
use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, List, ListItem};
use ratatui::Terminal;
use std::io::stdout;
use ratatui::style::{Style, Modifier, Color};

struct TodoItem {
    description: String,
    checked: bool,
}

struct TodoApp {
    items: Vec<TodoItem>,
    selected: usize,
}

impl TodoApp {
    fn new() -> Self {
        TodoApp {
            items: vec![
                TodoItem { description: "Task 1".to_string(), checked: false },
                TodoItem { description: "Task 2".to_string(), checked: false },
                TodoItem { description: "Task 3".to_string(), checked: true },
            ],
            selected: 0,
        }
    }
}

fn draw_ui<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &TodoApp) -> Result<(), Box<dyn std::error::Error>> {
    terminal.draw(|frame| {
        let size = frame.area();

        let items: Vec<ListItem> = app.items.iter().enumerate().map(|(i, item)| {
            let status = if item.checked { "✅" } else { "⭕️" };
            let content = format!("{} - {}", status, item.description);

            if i == app.selected {
                ListItem::new(content)
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            } else {
                ListItem::new(content)
            }
        }).collect();

        let list = List::new(items).block(Block::default().title("ToDo List").borders(Borders::ALL));

        frame.render_widget(list, size);
    })?;

    Ok(())
}

fn handle_input(app: &mut TodoApp) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if app.selected < app.items.len() - 1 {
                        app.selected += 1;
                    }
                }

                KeyCode::Left => {
                    if app.selected > 0 {
                        app.selected -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.selected < app.items.len() - 1 {
                        app.selected += 1;
                    }
                }

                // Blank enter to select
                KeyCode::Char(' ') => {
                    let item = &mut app.items[app.selected];
                    item.checked = !item.checked;
                }

                // Quit app
                KeyCode::Char('q') => return Ok(true), 
                _ => {}
            }
        }
    }

    Ok(false)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Handle user input manually
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let mut app = TodoApp::new();

    loop {
        draw_ui(&mut terminal, &app)?;
        if handle_input(&mut app)? {
            break;
        }
    }

    // Restore terminal state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}