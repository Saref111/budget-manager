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
    }, symbols::{block, DOT}, text::{Span, Spans}, widgets::{
        Block, Borders, List, ListItem, Paragraph, Tabs, Widget, Wrap
    }, Frame, Terminal
};

use unicode_width::UnicodeWidthStr;

use crate::types::{App, AppMode};

pub fn render(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App
) -> Result<(), io::Error>  {
    if app.mode == AppMode::Normal {
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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());
    
    let title = if app.entity.0.is_empty() { "Enter budget name:" } else { "Enter budget total amount of money:" };
    let edit_block = Paragraph::new(app.input.as_str())
        .block(Block::default().title(title).borders(Borders::ALL)
    ).wrap(Wrap { trim: true });

    let input_width = app.input.width() as u16;
    let lines_of_text = input_width / chunks[0].width;
    let char_offset = input_width % chunks[0].width;
    let horizontal_position = char_offset + (lines_of_text + lines_of_text) + 3;
    let vertical_position = lines_of_text + 3;

    f.set_cursor( horizontal_position, vertical_position);
    f.render_widget(edit_block, chunks[0]);
}