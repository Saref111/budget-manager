use std::{io, time::Duration};

use crossterm::event::{
    self, 
    Event, 
    KeyCode
};
use tui::widgets::ListState;

use crate::types::{
    App, AppMode, Budget, SavableBudget, UserActions
};


pub fn handle(
    app: &mut App
) -> Result<UserActions, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Left => {
                    if app.budgets.len() > 0 {
                        handle_key_left(&mut app.active_tab);
                    }
                },
                KeyCode::Right => {
                    if app.budgets.len() > 0 {
                        handle_key_right(&mut app.active_tab, app.budgets.len() - 1);
                    }
                },
                KeyCode::Up => {
                    if app.budgets.len() > 0 {
                        let current_budget = get_current_budget(&app);
                        let last_transaction_index = current_budget.transactions.len() - 1;
                        handle_key_up(&mut app.list_state, last_transaction_index);
                    }
                },
                KeyCode::Down => {
                    if app.budgets.len() > 0 {
                        let current_budget = get_current_budget(&app);
                        let last_transaction_index = current_budget.transactions.len() - 1;
                        handle_key_down(&mut app.list_state, last_transaction_index);
                    }
                },
                KeyCode::Backspace=> {
                    handle_backspace(&mut app.list_state);
                },
                KeyCode::Char('c') => {
                    app.mode = AppMode::InputNewBudget
                },
                KeyCode::Char('a') => {
                    if app.budgets.is_empty() {
                        return  Ok(UserActions::Continue);
                    }

                    app.mode = AppMode::InputNewTransaction(get_current_budget(&app).id);
                },
                KeyCode::Char('d') => {
                    if app.budgets.is_empty() {
                        return  Ok(UserActions::Continue);
                    }

                    return Ok(UserActions::RemoveBudget(get_current_budget(&app).id));
                },
                KeyCode::Char('u') => {
                    if app.budgets.is_empty() {
                        return Ok(UserActions::Continue);
                    }

                    let current_budget = app.budgets.get(app.active_tab).unwrap();
                    app.mode = AppMode::UpdateBudget(current_budget.id);

                    return Ok(UserActions::UpdateBudget(current_budget.get_without_transactions()));
                },
                KeyCode::Esc => return  Ok(UserActions::Exit),
                _ => {}
            }
        }
    }

    Ok(UserActions::Continue)
}

fn handle_key_left(active_tab: &mut usize) {
    if *active_tab > 0 {
        *active_tab -= 1;
    }
}

fn handle_key_right(active_tab: &mut usize, last_idx: usize) {
    if *active_tab < last_idx{
        *active_tab += 1;
    }
}

fn handle_key_down(list_state: &mut ListState, last_transaction_idx: usize) {
    if let Some(li) = list_state.selected() {
        if li < last_transaction_idx {
            list_state.select(Some(li + 1));
        } else {
            list_state.select(Some(0));
        }
    } else {
        list_state.select(Some(0));
    }
}

fn handle_key_up(list_state: &mut ListState, last_transaction_idx: usize) {
    if let Some(li) = list_state.selected() {
        if li > 0 {
            list_state.select(Some(li - 1));
        } else {
            list_state.select(Some(last_transaction_idx));
        }
    } else {
        list_state.select(Some(last_transaction_idx));
    }
}

fn handle_backspace(list_state: &mut ListState) {
    if let Some(_) = list_state.selected() {
        list_state.select(None);
    }
}

fn get_current_budget(app: &App) -> &Budget {
    app.budgets.get(app.active_tab).unwrap()
}