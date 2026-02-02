mod account_view;
mod input_screen;
mod network_selection;
mod styles;
mod transaction_view;

use crate::app::{App, Screen};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    match &app.screen {
        Screen::Input => input_screen::draw(f, app),
        Screen::NetworkSelection => network_selection::draw(f, app),
        Screen::Loading => draw_loading(f),
        Screen::Transaction => transaction_view::draw(f, app),
        Screen::Account => account_view::draw(f, app),
        Screen::Error(msg) => draw_error(f, msg),
    }
}

fn draw_loading(f: &mut Frame) {
    let size = f.size();
    let block = Block::default()
        .title(" Solana TUI ")
        .borders(Borders::ALL)
        .border_style(styles::PRIMARY_STYLE);

    f.render_widget(block, size);

    let loading_text = Paragraph::new("Loading...")
        .alignment(Alignment::Center)
        .style(styles::TEXT_STYLE);

    let area = centered_rect(30, 20, size);
    f.render_widget(Clear, area);
    f.render_widget(loading_text, area);
}

fn draw_error(f: &mut Frame, msg: &str) {
    let size = f.size();
    let block = Block::default()
        .title(" Error ")
        .borders(Borders::ALL)
        .border_style(styles::ERROR_STYLE);

    f.render_widget(block, size);

    let error_text = Paragraph::new(vec![
        Line::from(Span::styled("Error:", styles::ERROR_STYLE)),
        Line::from(""),
        Line::from(msg),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'r' to return or 'q' to quit",
            styles::HINT_STYLE,
        )),
    ])
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });

    let area = centered_rect(60, 40, size);
    f.render_widget(Clear, area);
    f.render_widget(error_text, area);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn format_sol(lamports: u64) -> String {
    format!("{:.9} SOL", lamports as f64 / 1_000_000_000.0)
}

pub fn truncate_pubkey(pubkey: &str) -> String {
    if pubkey.len() > 16 {
        format!("{}...{}", &pubkey[..8], &pubkey[pubkey.len() - 8..])
    } else {
        pubkey.to_string()
    }
}
