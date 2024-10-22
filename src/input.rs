use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::app::{TodoApp, TodoItem};

pub fn handle_input(app: &mut TodoApp) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if app.editing {
                match key.code {
                    KeyCode::Char(c) => {
                        app.current_input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.current_input.pop();
                    }
                    KeyCode::Enter => {
                        app.items[app.selected].description = app.current_input.clone();
                        app.editing = false;
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    // Task selection
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

                    // Enter Edit mode
                    KeyCode::Char('e') => {
                        app.editing = true;
                        app.current_input = app.items[app.selected].description.clone();
                    }

                    // Insert new task
                    KeyCode::Char('i') => {
                        let new_item = TodoItem {
                            description: String::from(""),
                            checked: false,
                        };
                        if app.items.len() == 0 {
                            app.items.insert(0, new_item);
                            app.selected = 0;
                        } else {
                            app.items.insert(app.selected + 1, new_item);
                            app.selected += 1;
                        }
                        app.editing = true;
                        app.current_input = String::new();
                    }

                    // Delete task
                    KeyCode::Char('d') => {
                        if app.items.len() > 1 {
                            app.items.remove(app.selected);
                            if app.selected >= app.items.len() {
                                app.selected = app.items.len() - 1;
                            }
                        } else if app.items.len() == 1 {
                            app.items.remove(app.selected);
                            app.selected = 0;
                        }
                    }

                    // Quit app
                    KeyCode::Char('q') => return Ok(true),
                    _ => {}
                }
            }
        }
    }

    Ok(false)
}