use crate::app::{App, TransactionTab};
use crate::solana::types::{TransactionData, TransactionStatus};
use crate::ui::styles::*;
use crate::ui::{format_sol, truncate_pubkey};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.size();
    let block = Block::default()
        .title(" Transaction Details ")
        .borders(Borders::ALL)
        .border_style(PRIMARY_STYLE);
    f.render_widget(block, size);

    if let Some(data) = &app.transaction_data {
        draw_transaction_content(f, data, app, size);
    } else {
        let no_data = Paragraph::new("No transaction data available")
            .alignment(ratatui::layout::Alignment::Center)
            .style(ERROR_STYLE);
        f.render_widget(no_data, size);
    }
}

fn draw_transaction_content(f: &mut Frame, data: &TransactionData, app: &App, area: Rect) {
    let inner = area.inner(&ratatui::layout::Margin {
        horizontal: 1,
        vertical: 1,
    });

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs
            Constraint::Min(0),    // Content
        ])
        .split(inner);

    draw_tabs(f, app, chunks[0]);

    match app.transaction_tab {
        TransactionTab::Overview => draw_overview(f, data, chunks[1]),
        TransactionTab::Accounts => draw_accounts(f, data, app.txn_scroll, chunks[1]),
        TransactionTab::Instructions => draw_instructions(f, data, app.txn_scroll, chunks[1]),
        TransactionTab::TokenTransfers => draw_token_transfers(f, data, app.txn_scroll, chunks[1]),
        TransactionTab::Logs => draw_logs(f, data, app.txn_scroll, chunks[1]),
    }
}

fn draw_tabs(f: &mut Frame, app: &App, area: Rect) {
    let titles = vec![
        TransactionTab::Overview,
        TransactionTab::Accounts,
        TransactionTab::Instructions,
        TransactionTab::TokenTransfers,
        TransactionTab::Logs,
    ]
    .into_iter()
    .map(|t| {
        let title = t.title();
        if t == app.transaction_tab {
            Line::from(vec![Span::styled(
                title,
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )])
        } else {
            Line::from(vec![Span::styled(title, Style::default().fg(Color::Gray))])
        }
    })
    .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::BOTTOM))
        .select(app.transaction_tab as usize)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        );

    f.render_widget(tabs, area);
}

fn draw_overview(f: &mut Frame, data: &TransactionData, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let status_style = match &data.status {
        TransactionStatus::Success => SUCCESS_STYLE,
        TransactionStatus::Failed(_) => ERROR_STYLE,
    };

    let status_text = match &data.status {
        TransactionStatus::Success => "✓ Success",
        TransactionStatus::Failed(e) => &format!("✗ Failed: {}", e),
    };

    let time_str = data
        .block_time
        .map(|t| t.format("%Y-%m-%d %H:%M:%S UTC").to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let sig_str = data.signature.to_string();

    let text = vec![
        Line::from(vec![
            Span::styled("Signature: ", HEADER_STYLE),
            Span::raw(&sig_str),
        ]),
        Line::from(vec![
            Span::styled("Slot: ", HEADER_STYLE),
            Span::raw(data.slot.to_string()),
        ]),
        Line::from(vec![
            Span::styled("Time: ", HEADER_STYLE),
            Span::raw(time_str),
        ]),
        Line::from(vec![
            Span::styled("Status: ", HEADER_STYLE),
            Span::styled(status_text.to_string(), status_style),
        ]),
        Line::from(vec![
            Span::styled("Fee: ", HEADER_STYLE),
            Span::raw(format_sol(data.fee)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Compute Units: ", HEADER_STYLE),
            Span::raw(format!(
                "{} / {}",
                data.compute_units_consumed.unwrap_or(0),
                data.max_compute_units.unwrap_or(200_000)
            )),
        ]),
        Line::from(vec![
            Span::styled("Priority Fee: ", HEADER_STYLE),
            Span::raw(format!(
                "{} micro-lamports",
                data.priority_fee.unwrap_or(0)
            )),
        ]),
    ];

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_accounts(f: &mut Frame, data: &TransactionData, scroll: usize, area: Rect) {
    let block = Block::default()
        .title(format!(" Accounts ({}) ", data.accounts.len()))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = Vec::new();

    for (i, acc) in data.accounts.iter().enumerate() {
        let balance_change = if let (Some(pre), Some(post)) = (acc.pre_balance, acc.post_balance) {
            let change = post as i64 - pre as i64;
            if change > 0 {
                format!(" (+{:.9} SOL)", change as f64 / 1_000_000_000.0)
            } else if change < 0 {
                format!(" ({:.9} SOL)", change as f64 / 1_000_000_000.0)
            } else {
                " (no change)".to_string()
            }
        } else {
            String::new()
        };

        let flags = format!(
            "{}{}",
            if acc.is_signer { "S" } else { " " },
            if acc.is_writable { "W" } else { " " }
        );

        let style = if balance_change.starts_with(" (+") {
            SUCCESS_STYLE
        } else if balance_change.starts_with(" (") && !balance_change.contains("no change") {
            ERROR_STYLE
        } else {
            DIM_STYLE
        };

        text.push(Line::from(vec![
            Span::styled(format!("{:<3} ", i), DIM_STYLE),
            Span::raw(flags),
            Span::raw(" "),
            Span::raw(truncate_pubkey(&acc.pubkey.to_string())),
            Span::styled(balance_change, style),
        ]));
    }

    // Scroll handling
    let visible_lines = area.height as usize - 2;
    let _total_lines = text.len();
    
    let display_text: Vec<Line> = text.into_iter().skip(scroll).take(visible_lines).collect();

    let paragraph = Paragraph::new(display_text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_instructions(f: &mut Frame, data: &TransactionData, scroll: usize, area: Rect) {
    let block = Block::default()
        .title(format!(" Instructions ({}) ", data.instructions.len()))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = Vec::new();

    for (i, ix) in data.instructions.iter().enumerate() {
        let program_name = ix.program_name.as_deref().unwrap_or("Unknown Program");
        
        text.push(Line::from(vec![
            Span::styled(format!("#{}: ", i + 1), HEADER_STYLE),
            Span::styled(program_name, Style::default().fg(Color::Cyan)),
            Span::raw(" > "),
            Span::styled(&ix.instruction_type, Style::default().fg(Color::Yellow)),
        ]));

        text.push(Line::from(vec![
             Span::raw("    Program ID: "),
             Span::raw(truncate_pubkey(&ix.program_id.to_string())),
        ]));

        text.push(Line::from(vec![
            Span::raw("    Data: "),
            Span::raw(if ix.data.len() > 50 { 
                format!("{}...", &ix.data[..50]) 
            } else { 
                ix.data.clone() 
            }),
        ]));
        
        text.push(Line::from("")); // Separator
    }

    let visible_lines = area.height as usize - 2;
    let display_text: Vec<Line> = text.into_iter().skip(scroll).take(visible_lines).collect();

    let paragraph = Paragraph::new(display_text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: false }); // False to avoid wrapping code/data weirdly

    f.render_widget(paragraph, area);
}

fn draw_token_transfers(f: &mut Frame, data: &TransactionData, scroll: usize, area: Rect) {
    let block = Block::default()
        .title(format!(" Token Transfers ({}) ", data.token_transfers.len()))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = Vec::new();

    if data.token_transfers.is_empty() {
        text.push(Line::from("No token transfers"));
    } else {
        for (i, transfer) in data.token_transfers.iter().enumerate() {
            let amount = transfer.amount as f64 / 10f64.powi(transfer.decimals as i32);
            
            text.push(Line::from(vec![
                Span::styled(format!("{}. ", i + 1), DIM_STYLE),
                Span::styled(format!("{:.4}", amount), SUCCESS_STYLE),
                Span::raw(" "),
                Span::raw(transfer.token_name.as_deref().unwrap_or("Token")),
            ]));

            text.push(Line::from(vec![
                Span::raw("   From: "),
                Span::raw(truncate_pubkey(&transfer.from.to_string())),
            ]));
            
            text.push(Line::from(vec![
                Span::raw("   To:   "),
                Span::raw(truncate_pubkey(&transfer.to.to_string())),
            ]));
            
            text.push(Line::from(""));
        }
    }

    let visible_lines = area.height as usize - 2;
    let display_text: Vec<Line> = text.into_iter().skip(scroll).take(visible_lines).collect();

    let paragraph = Paragraph::new(display_text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, area);
}

fn draw_logs(f: &mut Frame, data: &TransactionData, scroll: usize, area: Rect) {
    let block = Block::default()
        .title(format!(" Logs ({} lines) ", data.logs.len()))
        .borders(Borders::ALL)
        .border_style(SECONDARY_STYLE);

    let mut text: Vec<Line> = data
        .logs
        .iter()
        .skip(scroll)
        .take(area.height as usize - 2)
        .map(|log| Line::from(log.as_str()))
        .collect();

    if text.is_empty() && !data.logs.is_empty() {
        text.push(Line::from("Scroll up to see logs"));
    }

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(TEXT_STYLE)
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}