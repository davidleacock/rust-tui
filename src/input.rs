use crate::app::{App, TodoItem, Window};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};
use cli_log::*; // also import logging macros


pub fn handle_inputs(app: &mut App) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Tab => match app.active_window {
                    Window::TodoList => app.active_window = Window::Notepad,
                    Window::Notepad => app.active_window = Window::TodoList,
                },

                _ => {}
            }

            let result = match app.active_window {
                Window::TodoList => handle_todo_input(app, key)?,
                Window::Notepad => handle_notepad_input(app, key)?,
            };

            info!("Testing... {:#?}", app);

            return Ok(result);
        }
    }
    Ok(false)
}

fn handle_notepad_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    info!("Handle notepad input");
    if app.notepad_editing {
        match key.code {
            KeyCode::Char(c) => {
                app.notepad_content.push(c);
            }
            KeyCode::Backspace => {
                app.notepad_content.pop();
            }
            KeyCode::Enter => {
                app.notepad_content.push('\n');
            }

            _ => {}
        }
    } else {
        if let KeyCode::Char('e') = key.code {
            app.notepad_editing = !app.notepad_editing;
        }
    }

    Ok(false)
}

pub fn handle_todo_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    info!("Handle todo input");

    if event::poll(std::time::Duration::from_millis(10))? {
        if app.todo_editing {
            match key.code {
                KeyCode::Char(c) => {
                    app.current_input.push(c);
                }
                KeyCode::Backspace => {
                    app.current_input.pop();
                }
                KeyCode::Enter => {
                    app.items[app.selected].description = app.current_input.clone();
                    app.todo_editing = false;
                }
                _ => {}
            }
        } else {
            match key.code {
                // TodoItem selection
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
                    info!("entering edit mode");
                    app.todo_editing = true;
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
                    app.todo_editing = true;
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

    Ok(false)
}
