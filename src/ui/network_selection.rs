use crate::app::{App, InputType};
use crate::solana::Network;
use crate::ui::styles::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
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
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(size);

    let title = Paragraph::new("Solana Transaction & Account Explorer")
        .alignment(Alignment::Center)
        .style(HEADER_STYLE);
    f.render_widget(title, chunks[0]);

    // Show what was entered
    let input_type = match app.get_input_type() {
        InputType::Transaction => "Transaction",
        InputType::Account => "Account",
        InputType::Unknown => "Unknown",
    };

    let input_display = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Entered "),
            Span::styled(input_type, SUCCESS_STYLE),
            Span::raw(":"),
        ]),
        Line::from(app.input.as_str()),
    ])
    .alignment(Alignment::Center)
    .style(TEXT_STYLE);
    f.render_widget(input_display, chunks[2]);

    // Network selection prompt
    let prompt = Paragraph::new("Select Network:")
        .alignment(Alignment::Center)
        .style(TEXT_STYLE);
    f.render_widget(prompt, chunks[4]);

    // Network buttons - compact horizontal layout
    let network_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(chunks[5]);

    // Center the network selectors by using indices 1, 2, 3
    draw_network_button(
        f,
        Network::Mainnet,
        app.selected_network == Network::Mainnet,
        network_chunks[1],
    );
    draw_network_button(
        f,
        Network::Devnet,
        app.selected_network == Network::Devnet,
        network_chunks[2],
    );
    draw_network_button(
        f,
        Network::Testnet,
        app.selected_network == Network::Testnet,
        network_chunks[3],
    );

    // Hints
    let hints = Paragraph::new(vec![Line::from(vec![
        Span::styled("←/→", SELECTED_STYLE),
        Span::raw(" or "),
        Span::styled("↑/↓", SELECTED_STYLE),
        Span::raw(" to change  "),
        Span::styled("Enter", SELECTED_STYLE),
        Span::raw(" to confirm  "),
        Span::styled("Backspace", SELECTED_STYLE),
        Span::raw(" to go back"),
    ])])
    .alignment(Alignment::Center)
    .style(HINT_STYLE);
    f.render_widget(hints, chunks[7]);
}

fn draw_network_button(f: &mut Frame, network: Network, selected: bool, area: Rect) {
    let style = if selected { SELECTED_STYLE } else { DIM_STYLE };

    let block = Block::default()
        .title(format!(" {} ", network.name()))
        .borders(Borders::ALL)
        .border_style(style);

    let paragraph = Paragraph::new("").block(block).style(style);

    f.render_widget(paragraph, area);
}
