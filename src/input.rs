use crate::app::{App, TodoItem, Window};
use cli_log::*;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent}; // also import logging macros

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

            return Ok(result);
        }
    }
    Ok(false)
}

fn handle_notepad_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    if app.notepad_editing_mode {
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
        // TODO - fix this,  come up with more intuitive key commands based on widget context
        if let KeyCode::Char('e') = key.code {
            app.notepad_editing_mode = !app.notepad_editing_mode;
        }
    }

    Ok(false)
}

pub fn handle_todo_input(app: &mut App, key: KeyEvent) -> Result<bool, Box<dyn std::error::Error>> {
    if app.todo_editing_mode {
        match key.code {
            KeyCode::Char(c) => {
                app.current_todo_input.push(c);
            }
            KeyCode::Backspace => {
                app.current_todo_input.pop();
            }
            KeyCode::Enter => {
                app.todo_items[app.current_todo_item].description = app.current_todo_input.clone();
                app.todo_editing_mode = false;
            }
            _ => {}
        }
    } else {
        match key.code {
            // TodoItem selection
            KeyCode::Up => {
                if app.current_todo_item > 0 {
                    app.current_todo_item -= 1;
                }
            }
            KeyCode::Down => {
                if app.current_todo_item < app.todo_items.len() - 1 {
                    app.current_todo_item += 1;
                }
            }
            KeyCode::Left => {
                if app.current_todo_item > 0 {
                    app.current_todo_item -= 1;
                }
            }
            KeyCode::Right => {
                if app.current_todo_item < app.todo_items.len() - 1 {
                    app.current_todo_item += 1;
                }
            }

            // Blank enter to select
            KeyCode::Char(' ') => {
                let item = &mut app.todo_items[app.current_todo_item];
                item.checked = !item.checked;
            }

            // Enter Edit mode
            KeyCode::Char('e') => {
                app.todo_editing_mode = true;
                app.current_todo_input = app.todo_items[app.current_todo_item].description.clone();
            }

            // Insert new task
            KeyCode::Char('i') => {
                let new_item = TodoItem {
                    description: String::from(""),
                    checked: false,
                };
                if app.todo_items.len() == 0 {
                    app.todo_items.insert(0, new_item);
                    app.current_todo_item = 0;
                } else {
                    app.todo_items.insert(app.current_todo_item + 1, new_item);
                    app.current_todo_item += 1;
                }
                app.todo_editing_mode = true;
                app.current_todo_input = String::new();
            }

            // Delete task
            KeyCode::Char('d') => {
                if app.todo_items.len() > 1 {
                    app.todo_items.remove(app.current_todo_item);
                    if app.current_todo_item >= app.todo_items.len() {
                        app.current_todo_item = app.todo_items.len() - 1;
                    }
                } else if app.todo_items.len() == 1 {
                    app.todo_items.remove(app.current_todo_item);
                    app.current_todo_item = 0;
                }
            }

            // Quit app
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        }
    }

    Ok(false)
}
