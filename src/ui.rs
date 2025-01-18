use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::DOT, text::Spans, widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs, Widget}, Terminal
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
    let mut list_state = ListState::default();
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

            f.render_widget(tabs, chunks[0]);

            let content: Vec<ListItem> = budgets
                .get(active_tab)
                .unwrap()
                .transactions.iter()
                .enumerate()
                .map(|(i, t)| format!("{}. {}: ${}", i, t.message, t.sum))
                .map(ListItem::new)
                .collect();
            
            let list = List::new(content)
                .block(Block::default().title("Transactions").borders(Borders::ALL))
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol(">>");

            f.render_stateful_widget(list, chunks[1], &mut list_state);
        })?;

        if !(handle_interaction(&mut active_tab, &mut list_state, &budgets).unwrap()) {
            break;
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


fn handle_interaction(active_tab: &mut usize, list_state: &mut ListState, budgets: &Vec<Budget>) -> Result<bool, io::Error> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    if *active_tab > 0 {
                        *active_tab -= 1;
                    }
                },
                KeyCode::Right => {
                    if *active_tab < budgets.len() - 1 {
                        *active_tab += 1;
                    }
                },
                KeyCode::Up => {
                    if let Some(li) = list_state.selected() {
                        if li > 0 {
                            list_state.select(Some(li - 1));
                        } else {
                            list_state.select(Some(budgets.get(*active_tab).unwrap().transactions.len() - 1));
                        }
                    } else {
                        list_state.select(Some(budgets.get(*active_tab).unwrap().transactions.len() - 1));
                    }
                },
                KeyCode::Down => {
                    if let Some(li) = list_state.selected() {
                        if li < budgets.get(*active_tab).unwrap().transactions.len() - 1 {
                            list_state.select(Some(li + 1));
                        } else {
                            list_state.select(Some(0));
                        }
                    } else {
                        list_state.select(Some(0));
                    }
                },

                KeyCode::Esc => return  Ok(false),
                _ => {}
            }
        }
    }

    Ok(true)
}