use std::io;
use tui::{
    backend::CrosstermBackend, 
    widgets::ListState, 
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

use crate::{db::{budget, transaction}, types::{Budget, BudgetTransaction, MinimalBudget, PartialBudgetTransaction}};

use super::{handlers::handle_interaction, render::render};

pub enum UserActions {
    Exit,
    Continue,
    AddTransaction(PartialBudgetTransaction, u32),
    UpdateTransaction(BudgetTransaction),
    RemoveTransaction(u32),
    AddBudget(MinimalBudget),
}

pub fn run_ui(budgets: Vec<Budget>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut active_tab = 0;
    let mut list_state = ListState::default();

    loop {
        render(&mut terminal, &budgets, active_tab, &mut list_state)?;

        match handle_interaction(&mut active_tab, &mut list_state, &budgets)? {
            UserActions::Exit => break,
            UserActions::Continue => {},
            UserActions::AddTransaction(transaction, budget_id ) => {},
            UserActions::RemoveTransaction(id) => {},
            UserActions::UpdateTransaction(t) => {},
            UserActions::AddBudget(b) => {},
        }
    }
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

