use crate::app::App;
use crate::solana::types::{AccountData, TransactionStatus};
use crate::ui::styles::*;
use crate::ui::{format_sol, truncate_pubkey};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title(" Account Details ")
        .borders(Borders::ALL)
        .border_style(PRIMARY_STYLE);
    f.render_widget(block, size);

    if let Some(data) = &app.account_data {
        draw_account_content(f, data, app, size);
    } else {
        let no_data = Paragraph::new("No account data available")
            .alignment(ratatui::layout::Alignment::Center)
            .style(ERROR_STYLE);
        f.render_widget(no_data, size);
    }
}

fn draw_account_content(f: &mut Frame, data: &AccountData, app: &App, area: Rect) {
    let inner = area.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(8),
            Constraint::Min(0),
        ])
        .split(inner);

    draw_account_overview(f, data, chunks[0]);
    draw_token_accounts(f, data, chunks[1]);
    draw_transaction_history(f, data, app.account_scroll, chunks[2]);
}

fn draw_account_overview(f: &mut Frame, data: &AccountData, area: Rect) {
    let block = Block::default()
        .title(" Overview ")
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let account_type = if data.executable {
        "Program (Executable)"
    } else if data.data_size == 0 {
        "System Account"
    } else {
        "Data Account"
    };

    let pubkey_str = data.pubkey.to_string();
    let owner_str = data.owner.to_string();

    let text = vec![
        Line::from(vec![
            Span::styled("Address: ", HEADER_STYLE),
            Span::raw(&pubkey_str),
        ]),
        Line::from(vec![
            Span::styled("Balance: ", HEADER_STYLE),
            Span::styled(format_sol(data.lamports), SUCCESS_STYLE),
        ]),
        Line::from(vec![
            Span::styled("Owner: ", HEADER_STYLE),
            Span::raw(truncate_pubkey(&owner_str)),
        ]),
        Line::from(vec![
            Span::styled("Type: ", HEADER_STYLE),
            Span::raw(account_type),
            Span::raw("  Data Size: "),
            Span::raw(format!("{} bytes", data.data_size)),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_token_accounts(f: &mut Frame, data: &AccountData, area: Rect) {
    let block = Block::default()
        .title(format!(" Token Accounts ({}) ", data.token_accounts.len()))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = Vec::new();

    if data.token_accounts.is_empty() {
        text.push(Line::from("No token accounts found"));
    } else {
        for (i, token) in data.token_accounts.iter().enumerate() {
            let amount = token.amount as f64 / 10f64.powi(token.decimals as i32);
            let name = token.token_name.as_deref().unwrap_or("Unknown");
            let mint_str = token.mint.to_string();

            text.push(Line::from(vec![
                Span::styled(format!("{}. ", i + 1), DIM_STYLE),
                Span::raw(name),
                Span::raw(": "),
                Span::styled(format!("{:.6}", amount), SUCCESS_STYLE),
                Span::raw(" ("),
                Span::raw(truncate_pubkey(&mint_str)),
                Span::raw(")"),
            ]));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_transaction_history(f: &mut Frame, data: &AccountData, scroll: usize, area: Rect) {
    let block = Block::default()
        .title(format!(
            " Recent Transactions ({}) ",
            data.recent_transactions.len()
        ))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = Vec::new();

    if data.recent_transactions.is_empty() {
        text.push(Line::from("No recent transactions"));
    } else {
        let visible: Vec<_> = data
            .recent_transactions
            .iter()
            .skip(scroll)
            .take(area.height as usize - 2)
            .collect();

        for txn in visible {
            let time_str = txn
                .timestamp
                .as_ref()
                .map(|t| t.format("%m/%d %H:%M").to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let status_symbol = match &txn.status {
                TransactionStatus::Success => Span::styled("✓", SUCCESS_STYLE),
                TransactionStatus::Failed(_) => Span::styled("✗", ERROR_STYLE),
            };

            let sig_str = txn.signature.to_string();

            text.push(Line::from(vec![
                status_symbol,
                Span::raw(" "),
                Span::styled(time_str, DIM_STYLE),
                Span::raw(" Slot "),
                Span::raw(txn.slot.to_string()),
                Span::raw(" "),
                Span::raw(truncate_pubkey(&sig_str)),
            ]));
        }
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}
