use std::{io, time::Duration};

use crossterm::event::{self, Event, KeyCode};

use crate::types::{App, AppMode, UserActions};

pub fn handle(
    app: &mut App
) -> Result<UserActions, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Esc => {
                    app.mode = AppMode::Read
                },
                _ => {}
            }
        }
    }

    Ok(UserActions::Continue)
}