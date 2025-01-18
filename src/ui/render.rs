use std::io::{self, Stdout};

use tui::{
    backend::CrosstermBackend, 
    layout::{
        Constraint, 
        Direction, 
        Layout
    }, 
    style::{
        Color, 
        Modifier, 
        Style
    }, 
    symbols::DOT, 
    text::Spans, 
    widgets::{
        Block, 
        Borders, 
        List, 
        ListItem, 
        ListState, 
        Tabs
    }, 
    Terminal
};

use crate::types::Budget;

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    budgets: &Vec<Budget>,
    active_tab: usize,
    list_state: &mut ListState
) -> Result<(), io::Error>  {
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
            .map(|(i, t)| format!("{}. {}: ${}", i + 1, t.message, t.sum))
            .map(ListItem::new)
            .collect();
        
        let list = List::new(content)
            .block(Block::default().title("Transactions").borders(Borders::ALL))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">>");

        f.render_stateful_widget(list, chunks[1], list_state);
    })?;

    Ok(())
}