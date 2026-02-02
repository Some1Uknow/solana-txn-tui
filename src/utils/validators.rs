#![allow(dead_code)]
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use std::str::FromStr;

pub fn is_valid_pubkey(input: &str) -> bool {
    Pubkey::from_str(input).is_ok()
}

pub fn is_valid_signature(input: &str) -> bool {
    Signature::from_str(input).is_ok()
}
