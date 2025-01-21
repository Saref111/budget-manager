use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal
};
use crossterm::{
    event::{
        DisableMouseCapture, 
        EnableMouseCapture
    },
    execute,
    terminal::{
        disable_raw_mode, 
        enable_raw_mode, 
        EnterAlternateScreen, 
        LeaveAlternateScreen
    },
};

use crate::types::{
    App,
    UserActions
};

use super::{handlers::handle_interaction, render::render};

pub fn run_ui(mut app: App) -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        render(&mut terminal, &mut app)?;

        match handle_interaction(&mut app)? {
            UserActions::Exit => break,
            UserActions::Continue => {},
            UserActions::AddTransaction(transaction, budget_id ) => {},
            UserActions::RemoveTransaction(id) => {},
            UserActions::UpdateTransaction(t) => {},
            UserActions::AddBudget(b) => {},
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;


    Ok(())
}

