// Integration test for the specific transaction
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use std::str::FromStr;

fn main() {
    println!("Testing transaction: 4AeG6yqyqfRhJzBy2apTcCrVEDsEwqgHWsc8uFvdaKnseuYB8SjWC83KidujaELqe6sqGTUhdkK4eCzgNWWnbv3W");
    println!("Network: Devnet\n");

    let client = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com",
        CommitmentConfig::confirmed(),
    );

    let sig_str =
        "4AeG6yqyqfRhJzBy2apTcCrVEDsEwqgHWsc8uFvdaKnseuYB8SjWC83KidujaELqe6sqGTUhdkK4eCzgNWWnbv3W";

    match Signature::from_str(sig_str) {
        Ok(signature) => {
            println!("✓ Valid signature format");

            let config = solana_client::rpc_config::RpcTransactionConfig {
                encoding: Some(UiTransactionEncoding::JsonParsed),
                commitment: Some(CommitmentConfig::confirmed()),
                max_supported_transaction_version: Some(0),
            };

            match client.get_transaction_with_config(&signature, config) {
                Ok(txn) => {
                    println!("✓ Transaction found on devnet!");
                    println!("\n--- Transaction Details ---");
                    println!("  Signature: {}", sig_str);
                    println!("  Slot: {}", txn.slot);

                    if let Some(time) = txn.block_time {
                        let dt = chrono::DateTime::from_timestamp(time, 0);
                        println!("  Block Time: {:?}", dt);
                    }

                    if let Some(meta) = txn.transaction.meta {
                        let fee_sol = meta.fee as f64 / 1_000_000_000.0;
                        println!("  Fee: {} lamports ({:.9} SOL)", meta.fee, fee_sol);

                        if let Some(err) = &meta.err {
                            println!("  Status: ✗ Failed");
                            println!("  Error: {:?}", err);
                        } else {
                            println!("  Status: ✓ Success");
                        }

                        let logs: Vec<String> =
                            Option::from(meta.log_messages.clone()).unwrap_or_default();
                        println!("  Log Messages: {} lines", logs.len());

                        let compute_units: Option<u64> = Option::from(meta.compute_units_consumed);
                        if let Some(cu) = compute_units {
                            println!("  Compute Units: {}", cu);
                        }
                    }

                    println!("\n✓ TUI should display this transaction successfully!");
                }
                Err(e) => {
                    println!("✗ Failed to fetch transaction: {}", e);
                    println!(
                        "  The transaction may not exist or the RPC endpoint may be unavailable."
                    );
                }
            }
        }
        Err(e) => {
            println!("✗ Invalid signature format: {}", e);
        }
    }
}
