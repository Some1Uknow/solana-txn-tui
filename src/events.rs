use crate::app::{App, InputType, Screen};
use crate::solana::SolanaClient;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

pub fn handle_event(app: &mut App) -> anyhow::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return handle_key_event(app, key);
            }
        }
    }
    Ok(false)
}

fn handle_key_event(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match app.screen {
        Screen::Input => handle_input_screen(app, key),
        Screen::NetworkSelection => handle_network_selection_screen(app, key),
        Screen::Loading => Ok(false),
        Screen::Transaction => handle_transaction_screen(app, key),
        Screen::Account => handle_account_screen(app, key),
        Screen::Error(_) => handle_error_screen(app, key),
    }
}

fn handle_input_screen(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match key.code {
        // Only quit on Ctrl+C or Esc, NOT on 'q'
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
        KeyCode::Esc => return Ok(true),

        KeyCode::Char(c) => {
            app.insert_char(c);
        }
        KeyCode::Backspace => {
            app.delete_char();
        }
        KeyCode::Left => {
            app.move_cursor_left();
        }
        KeyCode::Right => {
            app.move_cursor_right();
        }
        KeyCode::Enter => {
            // Move to network selection after entering input
            if !app.input.is_empty() {
                let input_type = app.get_input_type();
                if input_type == InputType::Unknown {
                    app.screen = Screen::Error(
                        "Invalid input. Must be a transaction signature (88 chars) or a public key (32-44 chars)".to_string()
                    );
                } else {
                    app.screen = Screen::NetworkSelection;
                }
            }
        }
        _ => {}
    }
    Ok(false)
}

fn handle_network_selection_screen(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match key.code {
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
        KeyCode::Esc => {
            // Go back to input
            app.screen = Screen::Input;
        }
        KeyCode::Left => {
            app.selected_network = app.selected_network.prev();
        }
        KeyCode::Right => {
            app.selected_network = app.selected_network.next();
        }
        KeyCode::Up => {
            app.selected_network = app.selected_network.prev();
        }
        KeyCode::Down => {
            app.selected_network = app.selected_network.next();
        }
        KeyCode::Enter => {
            return submit_query(app);
        }
        KeyCode::Backspace => {
            // Go back to input
            app.screen = Screen::Input;
        }
        _ => {}
    }
    Ok(false)
}

fn submit_query(app: &mut App) -> anyhow::Result<bool> {
    let input_type = app.get_input_type();
    let input = app.input.clone();
    let network = app.selected_network;

    app.screen = Screen::Loading;

    match input_type {
        InputType::Transaction => {
            let client = SolanaClient::new(network);
            match client.fetch_transaction(&input) {
                Ok(data) => {
                    app.transaction_data = Some(data);
                    app.screen = Screen::Transaction;
                }
                Err(e) => {
                    app.screen = Screen::Error(format!("Failed to fetch transaction: {}", e));
                }
            }
        }
        InputType::Account => {
            let client = SolanaClient::new(network);
            match client.fetch_account(&input) {
                Ok(data) => {
                    app.account_data = Some(data);
                    app.screen = Screen::Account;
                }
                Err(e) => {
                    app.screen = Screen::Error(format!("Failed to fetch account: {}", e));
                }
            }
        }
        InputType::Unknown => {
            app.screen = Screen::Error(
                "Invalid input. Must be a transaction signature (88 chars) or a public key (32-44 chars)".to_string()
            );
        }
    }

    Ok(false)
}

fn handle_transaction_screen(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match key.code {
        // Only quit on Ctrl+C or Esc
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
        KeyCode::Esc => return Ok(true),
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.reset();
        }
        KeyCode::Up => {
            if app.txn_scroll > 0 {
                app.txn_scroll -= 1;
            }
        }
        KeyCode::Down => {
            app.txn_scroll += 1;
        }
        KeyCode::PageUp => {
            app.txn_scroll = app.txn_scroll.saturating_sub(10);
        }
        KeyCode::PageDown => {
            app.txn_scroll += 10;
        }
        KeyCode::Home => {
            app.txn_scroll = 0;
        }
        KeyCode::Tab => {
            app.transaction_tab = app.transaction_tab.next();
            app.txn_scroll = 0; // Reset scroll when switching tabs
        }
        KeyCode::BackTab => {
            app.transaction_tab = app.transaction_tab.prev();
            app.txn_scroll = 0;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_account_screen(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match key.code {
        // Only quit on Ctrl+C or Esc
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
        KeyCode::Esc => return Ok(true),
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.reset();
        }
        KeyCode::Up => {
            if app.account_scroll > 0 {
                app.account_scroll -= 1;
            }
        }
        KeyCode::Down => {
            app.account_scroll += 1;
        }
        KeyCode::PageUp => {
            app.account_scroll = app.account_scroll.saturating_sub(10);
        }
        KeyCode::PageDown => {
            app.account_scroll += 10;
        }
        KeyCode::Home => {
            app.account_scroll = 0;
        }
        _ => {}
    }
    Ok(false)
}

fn handle_error_screen(app: &mut App, key: KeyEvent) -> anyhow::Result<bool> {
    match key.code {
        // Only quit on Ctrl+C or Esc
        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(true),
        KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Enter | KeyCode::Esc => {
            app.reset();
        }
        _ => {}
    }
    Ok(false)
}
