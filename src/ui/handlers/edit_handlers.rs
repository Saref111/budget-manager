use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::types::{App, AppMode, MinimalBudget, UserActions};

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
                        app.mode = AppMode::Normal;
                        return Ok(UserActions::AddBudget(MinimalBudget {
                            total: app.entity.1.unwrap(),
                            name: app.entity.0.to_owned()
                        }));
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