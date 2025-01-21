use std::io::{
    self, 
    Stdout
};

use tui::{
    backend::CrosstermBackend, layout::{
        Constraint, 
        Direction, 
        Layout
    }, style::{
        Color, 
        Modifier, 
        Style
    }, symbols::DOT, text::{Span, Spans}, widgets::{
        Block, Borders, List, ListItem, Paragraph, Tabs
    }, Frame, Terminal
};

use crate::types::{App, AppMode};

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App
) -> Result<(), io::Error>  {
    if app.mode == AppMode::Read {
        terminal.draw(|f| draw_read_mode(f, app))?;
    } else {
        terminal.draw(|f| draw_edit_mode(f, app))?;
    }

    Ok(())
}

fn draw_read_mode(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());

        let titles = app.budgets.iter().map(|b| format!("{}: ${}", b.name, b.total)).map(Spans::from).collect();
        let tabs = Tabs::new(titles)
            .block(Block::default().title("Budget manager").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(DOT)
            .select(app.active_tab);

        f.render_widget(tabs, chunks[0]);

        let content: Vec<ListItem> = app.budgets
            .get(app.active_tab)
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

        f.render_stateful_widget(list, chunks[1], &mut app.list_state);
}

fn draw_edit_mode(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    f.render_widget(Paragraph::new("Edit"), f.size());
}