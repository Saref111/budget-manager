use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Style}, symbols::DOT, text::Spans, widgets::{Block, Borders, Paragraph, Tabs, Widget}, Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::types::Budget;

pub fn run_ui(budgets: Vec<Budget>) -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut active_tab = 0;
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            let titles = budgets.iter().map(|b| format!("{}: ${}", b.name, b.total)).map(Spans::from).collect();
            let tabs = Tabs::new(titles)
                .block(Block::default().title("Budget manager").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(DOT)
                .select(active_tab)
                ;

            let size = f.size();
            f.render_widget(tabs, chunks[0]);

            let content = match active_tab {
                0 => "This is the content of Tab 1",
                1 => "Content for Tab 2 goes here.",
                2 => "Here is what Tab 3 looks like.",
                3 => "Welcome to Tab 4!",
                _ => "Invalid Tab",
            };

            let paragraph = Paragraph::new(content)
                .block(Block::default().title("Content").borders(Borders::ALL));
            f.render_widget(paragraph, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => {
                        if active_tab >= 1 {
                            active_tab -= 1;
                        }
                    },

                    KeyCode::Right => {
                        if active_tab < budgets.len() - 1 {
                            active_tab += 1;
                        }
                    } 

                    KeyCode::Esc => break,
                    _ => {}
                }
            }
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
