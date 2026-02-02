use crate::solana::types::*;
use crate::solana::Network;
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedTransactionWithStatusMeta,
    UiCompiledInstruction, UiInstruction, UiParsedInstruction, UiTransactionEncoding,
};
use std::str::FromStr;

pub struct SolanaClient {
    client: RpcClient,
    network: Network,
}

impl std::fmt::Debug for SolanaClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SolanaClient")
            .field("network", &self.network)
            .finish_non_exhaustive()
    }
}

impl SolanaClient {
    pub fn new(network: Network) -> Self {
        let client = RpcClient::new_with_commitment(
            network.url().to_string(),
            CommitmentConfig::confirmed(),
        );
        Self { client, network }
    }

    #[allow(dead_code)]
    pub fn network(&self) -> Network {
        self.network
    }

    pub fn fetch_transaction(&self, signature_str: &str) -> Result<TransactionData> {
        let signature = Signature::from_str(signature_str)?;

        let config = solana_client::rpc_config::RpcTransactionConfig {
            encoding: Some(UiTransactionEncoding::JsonParsed),
            commitment: Some(CommitmentConfig::confirmed()),
            max_supported_transaction_version: Some(0),
        };

        let txn = self
            .client
            .get_transaction_with_config(&signature, config)?;

        self.parse_transaction(txn, signature)
    }

    pub fn fetch_account(&self, address_str: &str) -> Result<AccountData> {
        let pubkey = Pubkey::from_str(address_str)?;

        let account = self.client.get_account(&pubkey)?;

        let token_accounts = self.fetch_token_accounts(&pubkey)?;

        let signatures = self.client.get_signatures_for_address(&pubkey)?;
        let recent_transactions = signatures
            .into_iter()
            .take(10)
            .map(|sig| TransactionSummary {
                signature: Signature::from_str(&sig.signature).unwrap_or_default(),
                slot: sig.slot,
                timestamp: sig
                    .block_time
                    .and_then(|t| chrono::DateTime::from_timestamp(t, 0)),
                status: if let Some(err) = sig.err {
                    TransactionStatus::Failed(format!("{:?}", err))
                } else {
                    TransactionStatus::Success
                },
                fee: 0,
                description: String::new(),
            })
            .collect();

        Ok(AccountData {
            pubkey,
            lamports: account.lamports,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
            data_size: account.data.len(),
            token_accounts,
            recent_transactions,
            account_type: String::new(),
            is_rent_exempt: false,
            min_balance_for_rent_exemption: None,
        })
    }

    fn fetch_token_accounts(&self, owner: &Pubkey) -> Result<Vec<TokenAccountInfo>> {
        let token_accounts = self.client.get_token_accounts_by_owner(
            owner,
            solana_client::rpc_request::TokenAccountsFilter::ProgramId(
                solana_sdk::pubkey::Pubkey::from_str(
                    "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
                )?,
            ),
        )?;

        let mut result = Vec::new();
        for acc in token_accounts {
            if let solana_account_decoder::UiAccountData::Json(parsed) = acc.account.data {
                if let Ok(parsed_acc) = serde_json::from_value::<serde_json::Value>(parsed.parsed) {
                    if let Some(info) = parsed_acc.get("info") {
                        if let (Some(mint), Some(amount)) = (
                            info.get("mint").and_then(|m| m.as_str()),
                            info.get("tokenAmount")
                                .and_then(|t: &serde_json::Value| t.get("amount"))
                                .and_then(|a: &serde_json::Value| a.as_str()),
                        ) {
                            let decimals = info
                                .get("tokenAmount")
                                .and_then(|t: &serde_json::Value| t.get("decimals"))
                                .and_then(|d: &serde_json::Value| d.as_u64())
                                .unwrap_or(0) as u8;

                            let ui_amount = info
                                .get("tokenAmount")
                                .and_then(|t: &serde_json::Value| t.get("uiAmount"))
                                .and_then(|u| u.as_f64())
                                .unwrap_or(0.0);

                            result.push(TokenAccountInfo {
                                mint: Pubkey::from_str(mint).unwrap_or_default(),
                                amount: amount.parse::<u64>().unwrap_or(0),
                                decimals,
                                token_name: None,
                                ui_amount,
                            });
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    fn parse_transaction(
        &self,
        txn: EncodedConfirmedTransactionWithStatusMeta,
        signature: Signature,
    ) -> Result<TransactionData> {
        let meta = txn
            .transaction
            .meta
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No transaction metadata"))?
            .clone();
        let block_time = txn
            .block_time
            .and_then(|t| chrono::DateTime::from_timestamp(t, 0));

        let status = if let Some(err) = meta.err {
            TransactionStatus::Failed(format!("{:?}", err))
        } else {
            TransactionStatus::Success
        };

        // Get account keys for mapping indices to pubkeys
        let account_keys = match &txn.transaction.transaction {
            solana_transaction_status::EncodedTransaction::Json(parsed_txn) => {
                match &parsed_txn.message {
                    solana_transaction_status::UiMessage::Raw(raw_msg) => raw_msg
                        .account_keys
                        .iter()
                        .filter_map(|k| Pubkey::from_str(k).ok())
                        .collect::<Vec<_>>(),
                    solana_transaction_status::UiMessage::Parsed(parsed_msg) => parsed_msg
                        .account_keys
                        .iter()
                        .filter_map(|k| Pubkey::from_str(&k.pubkey).ok())
                        .collect::<Vec<_>>(),
                }
            }
            _ => Vec::new(),
        };

        // Parse instructions from the transaction
        let instructions = self.parse_instructions(&txn, &account_keys)?;

        // Parse inner instructions
        let inner_ix_option = match meta.inner_instructions.clone() {
            OptionSerializer::Some(ixs) => Some(ixs),
            _ => None,
        };
        let _inner_instructions = self.parse_inner_instructions(&inner_ix_option, &account_keys);

        // Parse token transfers from logs
        let logs_option = match meta.log_messages.clone() {
            OptionSerializer::Some(logs) => Some(logs),
            _ => None,
        };
        let token_transfers = self.parse_token_transfers_from_logs(&logs_option, &account_keys);

        // Parse SOL transfers from system program instructions
        let sol_transfers = self.parse_sol_transfers(&instructions, &account_keys);

        // Calculate priority fees from compute budget instructions
        let priority_fee = self.calculate_priority_fee(&instructions);

        // Extract accounts from the transaction message
        let accounts = match &txn.transaction.transaction {
            solana_transaction_status::EncodedTransaction::Json(parsed_txn) => {
                match &parsed_txn.message {
                    solana_transaction_status::UiMessage::Raw(raw_msg) => {
                        let account_keys = &raw_msg.account_keys;
                        let header = &raw_msg.header;

                        let num_required_signatures = header.num_required_signatures as usize;
                        let num_readonly_signed = header.num_readonly_signed_accounts as usize;
                        let num_readonly_unsigned = header.num_readonly_unsigned_accounts as usize;

                        let num_writable_signed =
                            num_required_signatures.saturating_sub(num_readonly_signed);
                        let num_writable_unsigned = account_keys
                            .len()
                            .saturating_sub(num_required_signatures)
                            .saturating_sub(num_readonly_unsigned);

                        account_keys
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, key_str)| {
                                let pubkey = Pubkey::from_str(key_str).ok()?;
                                let is_signer = idx < num_required_signatures;
                                let is_writable = (idx < num_writable_signed)
                                    || (idx >= num_required_signatures
                                        && idx < num_required_signatures + num_writable_unsigned);
                                let pre_balance = meta.pre_balances.get(idx).copied();
                                let post_balance = meta.post_balances.get(idx).copied();

                                Some(AccountMeta {
                                    pubkey,
                                    is_signer,
                                    is_writable,
                                    pre_balance,
                                    post_balance,
                                    account_type: None,
                                })
                            })
                            .collect()
                    }
                    solana_transaction_status::UiMessage::Parsed(parsed_msg) => {
                        // For parsed messages, we need to extract accounts from instructions
                        // and match them with balance changes from meta
                        parsed_msg
                            .account_keys
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, parsed_acc)| {
                                let pubkey = Pubkey::from_str(&parsed_acc.pubkey).ok()?;
                                let pre_balance = meta.pre_balances.get(idx).copied();
                                let post_balance = meta.post_balances.get(idx).copied();

                                Some(AccountMeta {
                                    pubkey,
                                    is_signer: parsed_acc.signer,
                                    is_writable: parsed_acc.writable,
                                    pre_balance,
                                    post_balance,
                                    account_type: None,
                                })
                            })
                            .collect()
                    }
                }
            }
            _ => Vec::new(),
        };

        // Get max compute units from compute budget instructions
        let max_compute_units = instructions
            .iter()
            .filter(|i| get_program_name(&i.program_id) == Some("Compute Budget"))
            .filter_map(|i| {
                if i.instruction_type.contains("SetComputeUnitLimit") {
                    i.data.parse::<u64>().ok()
                } else {
                    None
                }
            })
            .next();

        Ok(TransactionData {
            signature,
            slot: txn.slot,
            block_time,
            fee: meta.fee,
            status,
            instructions,
            accounts,
            logs: match meta.log_messages {
                OptionSerializer::Some(logs) => logs,
                _ => Vec::new(),
            },
            compute_units_consumed: match meta.compute_units_consumed {
                OptionSerializer::Some(units) => Some(units),
                _ => None,
            },
            version: txn.transaction.version.map(|v| format!("{:?}", v)),
            token_transfers,
            sol_transfers,
            priority_fee,
            max_compute_units,
        })
    }

    fn parse_instructions(
        &self,
        txn: &EncodedConfirmedTransactionWithStatusMeta,
        account_keys: &[Pubkey],
    ) -> Result<Vec<InstructionInfo>> {
        let mut instructions = Vec::new();

        match &txn.transaction.transaction {
            solana_transaction_status::EncodedTransaction::Json(parsed_txn) => {
                match &parsed_txn.message {
                    solana_transaction_status::UiMessage::Raw(raw_msg) => {
                        for (idx, ui_instr) in raw_msg.instructions.iter().enumerate() {
                            let instruction =
                                self.parse_raw_instruction(ui_instr, account_keys, idx)?;
                            instructions.push(instruction);
                        }
                    }
                    solana_transaction_status::UiMessage::Parsed(parsed_msg) => {
                        for (idx, ui_instr) in parsed_msg.instructions.iter().enumerate() {
                            let instruction = match ui_instr {
                                UiInstruction::Parsed(parsed) => {
                                    self.parse_parsed_instruction(parsed, idx)
                                }
                                UiInstruction::Compiled(compiled) => {
                                    // Should not happen in parsed message usually, but fallback
                                    self.parse_raw_instruction(compiled, account_keys, idx)
                                        .unwrap_or_else(|_| InstructionInfo {
                                            program_id: Pubkey::default(),
                                            program_name: None,
                                            instruction_type: "Unknown (Compiled)".to_string(),
                                            data: compiled.data.clone(),
                                            accounts: Vec::new(),
                                            compute_units_consumed: None,
                                        })
                                }
                            };
                            instructions.push(instruction);
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(instructions)
    }

    fn parse_raw_instruction(
        &self,
        ui_instr: &UiCompiledInstruction,
        account_keys: &[Pubkey],
        _idx: usize,
    ) -> Result<InstructionInfo> {
        let program_id = account_keys
            .get(ui_instr.program_id_index as usize)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("Invalid program_id_index"))?;

        let program_name = get_program_name(&program_id).map(|s| s.to_string());

        let instruction_type = self.identify_instruction_type(&program_id, &ui_instr.data);

        let accounts: Vec<AccountMeta> = ui_instr
            .accounts
            .iter()
            .filter_map(|&acc_idx| {
                account_keys
                    .get(acc_idx as usize)
                    .map(|&pubkey| AccountMeta {
                        pubkey,
                        is_signer: false, // Will be set based on transaction header
                        is_writable: false, // Will be set based on transaction header
                        pre_balance: None,
                        post_balance: None,
                        account_type: None,
                    })
            })
            .collect();

        // Try to decode base58 data
        let data_str = if let Ok(decoded) = bs58::decode(&ui_instr.data).into_vec() {
            let decoded: Vec<u8> = decoded;
            if decoded.len() >= 1 {
                format!("{} ({} bytes)", ui_instr.data, decoded.len())
            } else {
                ui_instr.data.clone()
            }
        } else {
            ui_instr.data.clone()
        };

        Ok(InstructionInfo {
            program_id,
            program_name,
            instruction_type,
            data: data_str,
            accounts,
            compute_units_consumed: None,
        })
    }

    fn parse_parsed_instruction(
        &self,
        ui_instr: &UiParsedInstruction,
        _idx: usize,
    ) -> InstructionInfo {
        match ui_instr {
            UiParsedInstruction::Parsed(parsed) => {
                let program_id = Pubkey::from_str(&parsed.program_id).unwrap_or_default();
                let program_name = get_program_name(&program_id).map(|s| s.to_string());

                let (instruction_type, data) = if let Ok(parsed_value) =
                    serde_json::from_value::<serde_json::Value>(parsed.parsed.clone())
                {
                    if let Some(instruction_type) =
                        parsed_value.get("type").and_then(|t| t.as_str())
                    {
                        let data = parsed_value
                            .get("info")
                            .map(|i| i.to_string())
                            .unwrap_or_default();
                        (instruction_type.to_string(), data)
                    } else {
                        ("Unknown".to_string(), parsed.parsed.to_string())
                    }
                } else {
                    ("Unknown".to_string(), parsed.parsed.to_string())
                };

                // Parse accounts from the parsed instruction
                let accounts: Vec<AccountMeta> = if let Ok(parsed_value) =
                    serde_json::from_value::<serde_json::Value>(parsed.parsed.clone())
                {
                    if let Some(info) = parsed_value.get("info") {
                        info.as_object()
                            .map(|obj| {
                                obj.iter()
                                    .filter_map(|(key, value)| {
                                        if let Some(pubkey_str) = value.as_str() {
                                            Pubkey::from_str(pubkey_str).ok().map(|pubkey| {
                                                AccountMeta {
                                                    pubkey,
                                                    is_signer: key.contains("authority")
                                                        || key.contains("owner"),
                                                    is_writable: key.contains("source")
                                                        || key.contains("destination"),
                                                    pre_balance: None,
                                                    post_balance: None,
                                                    account_type: Some(key.clone()),
                                                }
                                            })
                                        } else {
                                            None
                                        }
                                    })
                                    .collect()
                            })
                            .unwrap_or_default()
                    } else {
                        Vec::new()
                    }
                } else {
                    Vec::new()
                };

                InstructionInfo {
                    program_id,
                    program_name,
                    instruction_type,
                    data,
                    accounts,
                    compute_units_consumed: None,
                }
            }
            UiParsedInstruction::PartiallyDecoded(partial) => {
                let program_id = Pubkey::from_str(&partial.program_id).unwrap_or_default();
                let program_name = get_program_name(&program_id).map(|s| s.to_string());

                let accounts: Vec<AccountMeta> = partial
                    .accounts
                    .iter()
                    .filter_map(|acc_str| {
                        Pubkey::from_str(acc_str).ok().map(|pubkey| AccountMeta {
                            pubkey,
                            is_signer: false,
                            is_writable: false,
                            pre_balance: None,
                            post_balance: None,
                            account_type: None,
                        })
                    })
                    .collect();

                InstructionInfo {
                    program_id,
                    program_name,
                    instruction_type: "PartiallyDecoded".to_string(),
                    data: partial.data.clone(),
                    accounts,
                    compute_units_consumed: None,
                }
            }
        }
    }

    fn identify_instruction_type(&self, program_id: &Pubkey, data: &str) -> String {
        let program_name = get_program_name(program_id);

        match program_name {
            Some("System Program") => {
                // System program instructions: 0 = CreateAccount, 1 = Assign, 2 = Transfer, etc.
                if let Ok(decoded) = bs58::decode(data).into_vec() {
                    let decoded: Vec<u8> = decoded;
                    if !decoded.is_empty() {
                        match decoded[0] {
                            0 => "CreateAccount".to_string(),
                            1 => "Assign".to_string(),
                            2 => "Transfer".to_string(),
                            3 => "CreateAccountWithSeed".to_string(),
                            4 => "AdvanceNonceAccount".to_string(),
                            5 => "WithdrawNonceAccount".to_string(),
                            6 => "InitializeNonceAccount".to_string(),
                            7 => "AuthorizeNonceAccount".to_string(),
                            8 => "Allocate".to_string(),
                            9 => "AllocateWithSeed".to_string(),
                            10 => "AssignWithSeed".to_string(),
                            11 => "TransferWithSeed".to_string(),
                            12 => "UpgradeNonceAccount".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            }
            Some("Token Program") | Some("Token-2022 Program") => {
                // Token program instructions
                if let Ok(decoded) = bs58::decode(data).into_vec() {
                    let decoded: Vec<u8> = decoded;
                    if !decoded.is_empty() {
                        match decoded[0] {
                            0 => "InitializeMint".to_string(),
                            1 => "InitializeAccount".to_string(),
                            2 => "InitializeMultisig".to_string(),
                            3 => "Transfer".to_string(),
                            4 => "Approve".to_string(),
                            5 => "Revoke".to_string(),
                            6 => "SetAuthority".to_string(),
                            7 => "MintTo".to_string(),
                            8 => "Burn".to_string(),
                            9 => "CloseAccount".to_string(),
                            10 => "FreezeAccount".to_string(),
                            11 => "ThawAccount".to_string(),
                            12 => "TransferChecked".to_string(),
                            13 => "ApproveChecked".to_string(),
                            14 => "MintToChecked".to_string(),
                            15 => "BurnChecked".to_string(),
                            16 => "InitializeAccount2".to_string(),
                            17 => "SyncNative".to_string(),
                            18 => "InitializeAccount3".to_string(),
                            19 => "InitializeMultisig2".to_string(),
                            20 => "InitializeMint2".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            }
            Some("Compute Budget") => {
                // Compute budget instructions
                if let Ok(decoded) = bs58::decode(data).into_vec() {
                    let decoded: Vec<u8> = decoded;
                    if decoded.len() >= 1 {
                        match decoded[0] {
                            0 => "RequestUnits".to_string(),
                            1 => "RequestHeapFrame".to_string(),
                            2 => "SetComputeUnitLimit".to_string(),
                            3 => "SetComputeUnitPrice".to_string(),
                            4 => "SetLoadedAccountsDataSizeLimit".to_string(),
                            _ => "Unknown".to_string(),
                        }
                    } else {
                        "Unknown".to_string()
                    }
                } else {
                    "Unknown".to_string()
                }
            }
            _ => "Unknown".to_string(),
        }
    }

    fn parse_inner_instructions(
        &self,
        inner_instructions: &Option<Vec<solana_transaction_status::UiInnerInstructions>>,
        account_keys: &[Pubkey],
    ) -> Vec<InstructionInfo> {
        let mut result = Vec::new();

        if let Some(inner_ixs) = inner_instructions {
            for inner in inner_ixs {
                for (idx, ui_instr) in inner.instructions.iter().enumerate() {
                    match ui_instr {
                        UiInstruction::Compiled(compiled) => {
                            if let Ok(instruction) =
                                self.parse_raw_instruction(compiled, account_keys, idx)
                            {
                                result.push(instruction);
                            }
                        }
                        UiInstruction::Parsed(parsed) => {
                            let instruction = self.parse_parsed_instruction(parsed, idx);
                            result.push(instruction);
                        }
                    }
                }
            }
        }

        result
    }

    fn parse_token_transfers_from_logs(
        &self,
        logs: &Option<Vec<String>>,
        account_keys: &[Pubkey],
    ) -> Vec<TokenTransfer> {
        let mut transfers = Vec::new();

        if let Some(log_messages) = logs {
            for log in log_messages {
                // Parse TransferChecked events from token program logs
                if log.contains("Transfer") && log.contains("amount:") {
                    // Example log: "Program Tokenkeg... invoke [2]"
                    // "Program log: Instruction: TransferChecked"
                    // "Program log: TransferChecked 1000 from <from> to <to>, mint <mint>"
                    // This is simplified parsing - real implementation would be more robust

                    if let Some(transfer) = self.parse_transfer_log(log, account_keys) {
                        transfers.push(transfer);
                    }
                }

                // Parse Token:Transfer events
                if log.contains("Token:Transfer") {
                    // Parse Token:Transfer(amount, decimals)
                    if let Some(transfer) = self.parse_token_transfer_event(log, account_keys) {
                        transfers.push(transfer);
                    }
                }
            }
        }

        transfers
    }

    fn parse_transfer_log(&self, _log: &str, _account_keys: &[Pubkey]) -> Option<TokenTransfer> {
        // Simplified parsing - in reality, this would parse structured log data
        // For now, return a placeholder that can be enhanced
        None
    }

    fn parse_token_transfer_event(
        &self,
        _log: &str,
        _account_keys: &[Pubkey],
    ) -> Option<TokenTransfer> {
        // Parse Token:Transfer events from logs
        None
    }

    fn parse_sol_transfers(
        &self,
        instructions: &[InstructionInfo],
        _account_keys: &[Pubkey],
    ) -> Vec<SolTransfer> {
        let mut transfers = Vec::new();

        for instruction in instructions {
            let program_name = get_program_name(&instruction.program_id);

            // Check for System Program Transfer
            if program_name == Some("System Program") && instruction.instruction_type == "Transfer"
            {
                // Try to extract from and to accounts, and amount from data
                if instruction.accounts.len() >= 2 {
                    let from = instruction.accounts[0].pubkey;
                    let to = instruction.accounts[1].pubkey;

                    // Parse amount from data (base58 encoded)
                    // System transfer data format: [2, ...amount_bytes]
                    if let Ok(decoded) =
                        bs58::decode(&instruction.data.split_whitespace().next().unwrap_or(""))
                            .into_vec()
                    {
                        if decoded.len() >= 12 {
                            // Skip 4 bytes (instruction type), read 8 bytes for lamports
                            let amount = u64::from_le_bytes([
                                decoded[4],
                                decoded[5],
                                decoded[6],
                                decoded[7],
                                decoded[8],
                                decoded[9],
                                decoded[10],
                                decoded[11],
                            ]);
                            transfers.push(SolTransfer { from, to, amount });
                        }
                    }
                }
            }

            // Check for Token Program Transfer
            if (program_name == Some("Token Program") || program_name == Some("Token-2022 Program"))
                && (instruction.instruction_type == "Transfer"
                    || instruction.instruction_type == "TransferChecked")
            {
                // These are token transfers, already handled by parse_token_transfers_from_logs
                // But we could extract more details here if needed
            }
        }

        transfers
    }

    fn calculate_priority_fee(&self, instructions: &[InstructionInfo]) -> Option<u64> {
        let mut priority_fee = None;

        for instruction in instructions {
            let program_name = get_program_name(&instruction.program_id);

            if program_name == Some("Compute Budget") {
                // SetComputeUnitPrice instruction: data format [3, ...micro_lamports_bytes]
                if instruction.instruction_type == "SetComputeUnitPrice" {
                    if let Ok(decoded) =
                        bs58::decode(&instruction.data.split_whitespace().next().unwrap_or(""))
                            .into_vec()
                    {
                        if decoded.len() >= 9 {
                            // Skip 1 byte (instruction type), read 8 bytes for micro_lamports
                            let micro_lamports = u64::from_le_bytes([
                                decoded[1], decoded[2], decoded[3], decoded[4], decoded[5],
                                decoded[6], decoded[7], decoded[8],
                            ]);
                            priority_fee = Some(micro_lamports);
                        }
                    }
                }
            }
        }

        priority_fee
    }
}
