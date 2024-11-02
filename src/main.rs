mod app;
mod input;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::stdout;

use crate::app::*;
use crate::input::*;
use crate::ui::*;
use cli_log::*;         


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init_cli_log!(); 
    
    // Handle user input manually
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    loop {
        draw_ui(&mut terminal, &app)?;
        if handle_inputs(&mut app)? {
            break;
        }
    }

    // Restore terminal state
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
