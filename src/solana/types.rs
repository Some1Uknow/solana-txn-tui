#![allow(dead_code)]
use chrono::{DateTime, Utc};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[derive(Debug, Clone)]
pub struct TransactionData {
    pub signature: Signature,
    pub slot: u64,
    pub block_time: Option<DateTime<Utc>>,
    pub fee: u64,
    pub status: TransactionStatus,
    pub instructions: Vec<InstructionInfo>,
    pub accounts: Vec<AccountMeta>,
    pub logs: Vec<String>,
    pub compute_units_consumed: Option<u64>,
    pub version: Option<String>,
    pub token_transfers: Vec<TokenTransfer>,
    pub sol_transfers: Vec<SolTransfer>,
    pub priority_fee: Option<u64>,
    pub max_compute_units: Option<u64>,
}

#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Success,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct InstructionInfo {
    pub program_id: Pubkey,
    pub program_name: Option<String>,
    pub instruction_type: String,
    pub data: String,
    pub accounts: Vec<AccountMeta>,
    pub compute_units_consumed: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
    pub pre_balance: Option<u64>,
    pub post_balance: Option<u64>,
    pub account_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TokenTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
    pub token_name: Option<String>,
    pub program: String,
}

#[derive(Debug, Clone)]
pub struct SolTransfer {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct AccountData {
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: u64,
    pub data_size: usize,
    pub token_accounts: Vec<TokenAccountInfo>,
    pub recent_transactions: Vec<TransactionSummary>,
    pub account_type: String,
    pub is_rent_exempt: bool,
    pub min_balance_for_rent_exemption: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct TokenAccountInfo {
    pub mint: Pubkey,
    pub amount: u64,
    pub decimals: u8,
    pub token_name: Option<String>,
    pub ui_amount: f64,
}

#[derive(Debug, Clone)]
pub struct TransactionSummary {
    pub signature: Signature,
    pub slot: u64,
    pub timestamp: Option<DateTime<Utc>>,
    pub status: TransactionStatus,
    pub fee: u64,
    pub description: String,
}

// Known program IDs and their names
pub fn get_program_name(program_id: &Pubkey) -> Option<&'static str> {
    const KNOWN_PROGRAMS: &[(&str, &str)] = &[
        ("11111111111111111111111111111111", "System Program"),
        (
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "Token Program",
        ),
        (
            "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCQbphWkTg",
            "Token-2022 Program",
        ),
        (
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            "Associated Token Account",
        ),
        (
            " ComputeBudget111111111111111111111111111111",
            "Compute Budget",
        ),
        (
            "Config1111111111111111111111111111111111111",
            "Config Program",
        ),
        (
            "Stake11111111111111111111111111111111111111",
            "Stake Program",
        ),
        (
            "Vote111111111111111111111111111111111111111",
            "Vote Program",
        ),
        (
            "AddressLookupTab1e1111111111111111111111111",
            "Address Lookup Table",
        ),
        (
            "BPFLoaderUpgradeab1e11111111111111111111111",
            "BPF Loader Upgradeable",
        ),
        ("BPFLoader2111111111111111111111111111111111", "BPF Loader"),
        (
            "BPFLoader1111111111111111111111111111111111",
            "BPF Loader (Legacy)",
        ),
        (
            "Ed25519SigVerify111111111111111111111111111",
            "Ed25519 SigVerify",
        ),
        (
            "KeccakSecp256k11111111111111111111111111111",
            "Secp256k1 Program",
        ),
    ];

    let program_id_str = program_id.to_string();
    KNOWN_PROGRAMS
        .iter()
        .find(|(id, _)| *id == program_id_str)
        .map(|(_, name)| *name)
}
