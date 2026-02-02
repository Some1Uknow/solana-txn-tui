use crate::solana::{Network, SolanaClient};
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Input,
    NetworkSelection,
    Loading,
    Transaction,
    Account,
    Error(String),
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub input: String,
    pub input_cursor: usize,
    pub selected_network: Network,
    pub error_message: Option<String>,
    #[allow(dead_code)]
    pub solana_client: Option<SolanaClient>,
    pub transaction_data: Option<solana::TransactionData>,
    pub account_data: Option<solana::AccountData>,
    pub txn_scroll: usize,
    pub account_scroll: usize,
    pub transaction_tab: TransactionTab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransactionTab {
    Overview,
    Accounts,
    Instructions,
    TokenTransfers,
    Logs,
}

impl TransactionTab {
    pub fn next(&self) -> Self {
        match self {
            Self::Overview => Self::Accounts,
            Self::Accounts => Self::Instructions,
            Self::Instructions => Self::TokenTransfers,
            Self::TokenTransfers => Self::Logs,
            Self::Logs => Self::Overview,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::Overview => Self::Logs,
            Self::Accounts => Self::Overview,
            Self::Instructions => Self::Accounts,
            Self::TokenTransfers => Self::Instructions,
            Self::Logs => Self::TokenTransfers,
        }
    }
    
    pub fn title(&self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Accounts => "Accounts",
            Self::Instructions => "Instructions",
            Self::TokenTransfers => "Token Transfers",
            Self::Logs => "Logs",
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Input,
            input: String::new(),
            input_cursor: 0,
            selected_network: Network::Mainnet,
            error_message: None,
            solana_client: None,
            transaction_data: None,
            account_data: None,
            txn_scroll: 0,
            account_scroll: 0,
            transaction_tab: TransactionTab::Overview,
        }
    }

    pub fn get_input_type(&self) -> InputType {
        let trimmed = self.input.trim();

        if trimmed.len() == 88 || trimmed.len() == 87 {
            if Signature::from_str(trimmed).is_ok() {
                return InputType::Transaction;
            }
        }

        if Pubkey::from_str(trimmed).is_ok() {
            return InputType::Account;
        }

        InputType::Unknown
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.input_cursor, c);
        self.input_cursor += 1;
    }

    pub fn delete_char(&mut self) {
        if self.input_cursor > 0 {
            self.input.remove(self.input_cursor - 1);
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input.len() {
            self.input_cursor += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor = 0;
    }

    pub fn reset(&mut self) {
        self.screen = Screen::Input;
        self.clear_input();
        self.error_message = None;
        self.transaction_data = None;
        self.account_data = None;
        self.txn_scroll = 0;
        self.account_scroll = 0;
        self.transaction_tab = TransactionTab::Overview;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputType {
    Transaction,
    Account,
    Unknown,
}

pub mod solana {
    pub use super::super::solana::*;
}
