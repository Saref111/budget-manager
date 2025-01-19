use std::{io, time::Duration};

use crossterm::{event::{self, EnableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen}};
use tui::widgets::ListState;

use crate::{db::transaction::add_transaction, types::{Budget, MinimalBudget, PartialBudgetTransaction}};

use super::{prompts::{prompt_for_new_budget, prompt_for_new_transaction}, ui::UserActions};

pub fn handle_interaction(
    active_tab: &mut usize, 
    list_state: &mut ListState, 
    budgets: &Vec<Budget>
) -> Result<UserActions, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    handle_key_left(active_tab);
                },
                KeyCode::Right => {
                    handle_key_right(active_tab, budgets.len() - 1);
                },
                KeyCode::Up => {
                    handle_key_up(list_state, budgets.get(*active_tab).unwrap().transactions.len() - 1);
                },
                KeyCode::Down => {
                    handle_key_down(list_state, budgets.get(*active_tab).unwrap().transactions.len() - 1);
                },
                KeyCode::Backspace=> {
                    handle_backspace(list_state);
                },
                KeyCode::Char('b') => {
                    return Ok(UserActions::AddBudget(handle_b_char()?));
                },
                KeyCode::Char('t') => {
                    if !budgets.is_empty() {
                        return Ok(UserActions::AddTransaction(handle_t_char()?, budgets.get(*active_tab).unwrap().id));
                    }
                },
                KeyCode::Char('r') => {},
                KeyCode::Char('u') => {},
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

fn handle_b_char() -> Result<MinimalBudget, io::Error>  {
    disable_raw_mode()?;
    let b = prompt_for_new_budget();

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    Ok(b)
}

fn handle_t_char() -> Result<PartialBudgetTransaction, io::Error>  {
    disable_raw_mode()?;
    let t = prompt_for_new_transaction();

    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    Ok(t)
}