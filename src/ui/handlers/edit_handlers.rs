use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::types::{
    App, 
    AppMode, 
    BudgetTransaction, 
    MinimalBudget, 
    PartialBudget, 
    PartialBudgetTransaction,
    UserActions
};

pub fn handle(
    app: &mut App
) -> Result<UserActions, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    app.mode = AppMode::Normal
                },
                KeyCode::Enter => {
                    if app.input.is_empty() {
                        return Ok(UserActions::Continue)
                    };

                    if app.entity.0.is_empty() { 
                        app.entity.0 = app.input.to_owned();
                        app.input = String::new();
                    } else if app.entity.1.is_none() {
                        app.entity.1 = app.input.parse::<i32>().ok();
                        app.input = String::new();
                    }
                    
                    if !app.entity.0.is_empty() && app.entity.1.is_some() {
                        let entity_action = match app.mode {
                            AppMode::UpdateTransaction(id) => UserActions::UpdateTransaction(BudgetTransaction {
                                id: id.to_string(),
                                message: app.entity.0.to_owned(),
                                sum: app.entity.1.unwrap()
                            }),
                            AppMode::InputNewTransaction(id) => UserActions::AddTransaction(PartialBudgetTransaction {
                                message: app.entity.0.to_owned(),
                                sum: app.entity.1.unwrap(),
                            }, id),
                            AppMode::UpdateBudget(id) => UserActions::UpdateBudget(PartialBudget {
                                total: app.entity.1.unwrap(),
                                name: app.entity.0.to_owned(),
                                id
                            }),
                            _ => UserActions::AddBudget(MinimalBudget {
                                total: app.entity.1.unwrap(),
                                name: app.entity.0.to_owned()
                            }),
                        };

                        app.mode = AppMode::Normal;
                        app.entity.0 = String::new();
                        app.entity.1 = None;
                        return Ok(entity_action);
                    }

                }
                KeyCode::Char(c) => {
                    app.input.push(c);
                }
                KeyCode::Backspace => {
                    app.input.pop();
                }
                _ => {}
            }
        }
    }

    Ok(UserActions::Continue)
}