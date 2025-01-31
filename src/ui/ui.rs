use std::io;
use tui::{
    backend::CrosstermBackend,
    Terminal
};

use std::error::Error;
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
    App, AppMode, UserActions
};

use super::{
    handlers::navigation_handlers::handle as handle_nav, 
    handlers::edit_handlers::handle as handle_edit, 
    render::render
};

pub fn run(mut app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        render(&mut terminal, &mut app)?;

        if app.mode == AppMode::Normal {

            match handle_nav(&mut app)? {
                UserActions::Exit => break,
                UserActions::Continue => {},
                UserActions::RemoveBudget(id) => {
                    app.remove_budget(id)?;
                },
                UserActions::RemoveTransaction(id) => {
                    app.remove_transaction(id)?;
                }
                _ => {}
            }

        } else {
            match handle_edit(&mut app)? {
                UserActions::AddTransaction(t, id) => {
                    app.add_new_transaction(t, id)?;
                },
                UserActions::UpdateTransaction(t) => {
                    app.update_transaction(t)?;
                },
                UserActions::AddBudget(b) => {
                    app.add_new_budget(b)?;
                },
                UserActions::UpdateBudget(b) => {
                    app.update_budget(b)?;
                },
                _ => {}
            }
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

