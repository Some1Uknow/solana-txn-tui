use crate::app::{App, InputType};
use crate::ui::styles::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Length(2),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("Solana Transaction & Account Explorer")
        .alignment(Alignment::Center)
        .style(HEADER_STYLE.add_modifier(Modifier::BOLD));
    f.render_widget(title, chunks[0]);

    let input_type = match app.get_input_type() {
        InputType::Transaction => Span::styled("Transaction", SUCCESS_STYLE),
        InputType::Account => Span::styled("Account", SUCCESS_STYLE),
        InputType::Unknown => Span::styled("Unknown", ERROR_STYLE),
    };

    let input_block = Block::default()
        .title(Line::from(vec![
            Span::raw(" Input ("),
            input_type,
            Span::raw(") "),
        ]))
        .borders(Borders::ALL)
        .border_style(PRIMARY_STYLE);

    let input_text = Paragraph::new(app.input.as_str())
        .block(input_block)
        .style(TEXT_STYLE);
    f.render_widget(input_text, chunks[2]);

    let cursor_x = chunks[2].x + app.input_cursor as u16 + 1;
    let cursor_y = chunks[2].y + 1;
    f.set_cursor(cursor_x, cursor_y);

    let hints = Paragraph::new(vec![Line::from(vec![
        Span::styled("Enter", SELECTED_STYLE),
        Span::raw(" to continue  "),
        Span::styled("Ctrl+C", SELECTED_STYLE),
        Span::raw(" or "),
        Span::styled("Esc", SELECTED_STYLE),
        Span::raw(" to quit"),
    ])])
    .alignment(Alignment::Center)
    .style(HINT_STYLE);
    f.render_widget(hints, chunks[4]);
}
