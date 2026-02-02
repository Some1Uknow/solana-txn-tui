pub mod client;
pub mod types;

pub use client::SolanaClient;
pub use types::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Network {
    Mainnet,
    Devnet,
    Testnet,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Testnet => "https://api.testnet.solana.com",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Network::Mainnet => "Mainnet",
            Network::Devnet => "Devnet",
            Network::Testnet => "Testnet",
        }
    }

    pub fn next(&self) -> Network {
        match self {
            Network::Mainnet => Network::Devnet,
            Network::Devnet => Network::Testnet,
            Network::Testnet => Network::Mainnet,
        }
    }

    pub fn prev(&self) -> Network {
        match self {
            Network::Mainnet => Network::Testnet,
            Network::Devnet => Network::Mainnet,
            Network::Testnet => Network::Devnet,
        }
    }
}
