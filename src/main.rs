mod app;
mod input;
mod ui;

use crossterm::{execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

use crate::app::*;
use crate::input::*;
use crate::ui::*;

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