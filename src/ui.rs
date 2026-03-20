use crate::app::{App, InputMode, NamingMode};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let filter_text = match app.filter {
        crate::models::Filter::All => "All",
        crate::models::Filter::Unwatched => "Unwatched",
        crate::models::Filter::Watched => "Watched",
    };

    let title = Paragraph::new(Line::from(vec![
        Span::raw("🎬 "),
        Span::raw("Film List"),
        Span::raw(" | "),
        Span::styled(filter_text, Style::new().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Film List"));

    frame.render_widget(title, chunks[0]);

    let films = app.filtered_films();
    let list_items: Vec<ListItem> = films
        .iter()
        .enumerate()
        .map(|(i, film)| {
            let watched_emoji = if film.watched { "✅" } else { "⬜" };
            let rating_str = film.rating.map(|r| format!("⭐{}", r)).unwrap_or_default();
            let content = format!("{} {} {}", watched_emoji, film.name, rating_str);
            let style = if i == app.selected_index {
                Style::new().bg(Color::Blue)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Films"))
        .highlight_style(Style::new().bg(Color::Blue));

    frame.render_widget(list, chunks[1]);

    let help_text = Paragraph::new(
        "↑↓ Navigate | a: Add | d: Delete | w: Toggle watched | f: Filter | q: Quit",
    )
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(help_text, chunks[2]);

    if !(matches!(app.input_mode, InputMode::Normal) || matches!(app.input_mode, InputMode::Breakdown)) {
        let title = match app.input_mode {
            InputMode::Naming(NamingMode::Creating) => String::from("Name Film"),
            InputMode::Naming(NamingMode::Modifying) => String::from("Rename Film"),
            InputMode::Rating => String::from("Rate film (1-10)"),
            InputMode::Director => String::from("Add Director"),
            _ => String::from("an error occured"),
        };
        let popup = Paragraph::new(app.input_value.as_str())
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Yellow)),
            )
            .style(Style::new().bg(Color::Black));

        let area = centered_rect(60, 3, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(popup, area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let mid_x = r.width / 2 - percent_x / 2;
    let mid_y = r.height / 2 - percent_y / 2;
    Rect::new(mid_x, mid_y, percent_x, percent_y)
}
