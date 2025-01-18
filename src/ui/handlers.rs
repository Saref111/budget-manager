use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use tui::widgets::ListState;

use crate::types::Budget;

pub fn handle_interaction(
    active_tab: &mut usize, 
    list_state: &mut ListState, 
    budgets: &Vec<Budget>
) -> Result<bool, io::Error> {
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

                KeyCode::Esc => return  Ok(false),
                _ => {}
            }
        }
    }

    Ok(true)
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