use crate::domain::{Block, Transaction};
use chrono::{DateTime, NaiveDateTime, Utc};
use ethers::types::{Block as EthersBlock, Transaction as EthersTransaction, U64, U256};
use serde_json::Value;

pub fn ethers_block_to_block(ethers_block: EthersBlock<EthersTransaction>) -> Option<Block> {
    let timestamp = ethers_block.timestamp.as_u64();

    Some(Block {
        number: ethers_block.number?.as_u64(),
        hash: format!("{:#x}", ethers_block.hash?),
        parent_hash: format!("{:#x}", ethers_block.parent_hash),
        nonce: format!("{:#x}", ethers_block.nonce?),
        sha3_uncles: format!("{:#x}", ethers_block.uncles_hash),
        logs_bloom: format!("{:#x}", ethers_block.logs_bloom?),
        transactions_root: format!("{:#x}", ethers_block.transactions_root),
        state_root: format!("{:#x}", ethers_block.state_root),
        receipts_root: format!("{:#x}", ethers_block.receipts_root),
        miner: format!("{:#x}", ethers_block.author?),
        difficulty: ethers_block.difficulty.to_string(),
        total_difficulty: ethers_block.total_difficulty.map(|val| val.to_string()),
        size: ethers_block.size?.as_u64(),
        extra_data: format!("{:#x}", ethers_block.extra_data),
        gas_limit: ethers_block.gas_limit.as_u64(),
        gas_used: ethers_block.gas_used.as_u64(),
        timestamp,
        transaction_count: ethers_block.transactions.len() as u64,
        base_fee_per_gas: ethers_block.base_fee_per_gas.map(|v| v.as_u64()),
        withdrawals_root: ethers_block.withdrawals_root.map(|v| format!("{:#x}", v)),
        blob_gas_used: ethers_block.blob_gas_used.map(|v| v.as_u64()),
        excess_blob_gas: ethers_block.excess_blob_gas.map(|v| v.as_u64()),
        transactions: ethers_block
            .transactions
            .into_iter()
            .map(|tx| ethers_transaction_to_transaction(tx, timestamp))
            .collect(),
    })
}

pub fn ethers_transaction_to_transaction(
    ethers_tx: EthersTransaction,
    block_timestamp: u64,
) -> Transaction {
    let other_fields = serde_json::to_value(ethers_tx.other).unwrap_or_default();

    Transaction {
        hash: format!("{:#x}", ethers_tx.hash),
        nonce: ethers_tx.nonce.as_u64(),
        block_hash: ethers_tx.block_hash.map(|h| format!("{:#x}", h)),
        block_number: ethers_tx.block_number.map(|bn| bn.as_u64()),
        transaction_index: ethers_tx.transaction_index.map(|i| i.as_u64()),
        from_address: format!("{:#x}", ethers_tx.from),
        to_address: ethers_tx.to.map(|addr| format!("{:#x}", addr)),
        value: ethers_tx.value.to_string(),
        gas: ethers_tx.gas.as_u64(),
        gas_price: ethers_tx.gas_price.unwrap_or_default().to_string(),
        input: format!("{:#x}", ethers_tx.input),
        block_timestamp,
        max_fee_per_gas: ethers_tx.max_fee_per_gas.map(|v| v.as_u64()),
        max_priority_fee_per_gas: ethers_tx.max_priority_fee_per_gas.map(|v| v.as_u64()),
        transaction_type: ethers_tx.transaction_type.map(|v| v.as_u64()),
        max_fee_per_blob_gas: other_fields.get("maxFeePerBlobGas").and_then(|v| v.as_str()).and_then(|s| s.parse::<u64>().ok()),
        blob_versioned_hashes: other_fields.get("blobVersionedHashes").and_then(|v| v.as_array()).map(|hashes| {
            hashes.iter().filter_map(|h| h.as_str().map(|s| s.to_string())).collect()
        }).unwrap_or_default(),
    }
}

pub fn block_to_csv_row(block: &Block) -> Vec<String> {
    vec![
        block.number.to_string(),
        block.hash.clone(),
        block.parent_hash.clone(),
        block.nonce.clone(),
        block.sha3_uncles.clone(),
        block.logs_bloom.clone(),
        block.transactions_root.clone(),
        block.state_root.clone(),
        block.receipts_root.clone(),
        block.miner.clone(),
        block.difficulty.clone(),
        block.total_difficulty.clone().unwrap_or_else(|| "0".to_string()),
        block.size.to_string(),
        block.extra_data.clone(),
        block.gas_limit.to_string(),
        block.gas_used.to_string(),
        block.timestamp.to_string(),
        block.transaction_count.to_string(),
        block.base_fee_per_gas.map(|num| num.to_string()).unwrap_or_default(),
        block.withdrawals_root.clone().unwrap_or_else(|| "".to_string()),
        block.blob_gas_used.map(|num| num.to_string()).unwrap_or_default(),
        block.excess_blob_gas.map(|num| num.to_string()).unwrap_or_default(),
    ]
}

pub fn transaction_to_csv_row(transaction: &Transaction) -> Vec<String> {
    vec![
        transaction.hash.clone(),
        transaction.nonce.to_string(),
        transaction.block_hash.clone().unwrap_or_else(|| "".to_string()),
        transaction.block_number.map(|num| num.to_string()).unwrap_or_default(),
        transaction.transaction_index.map(|num| num.to_string()).unwrap_or_default(),
        transaction.from_address.clone(),
        transaction.to_address.clone().unwrap_or_else(|| "".to_string()),
        transaction.value.clone(),
        transaction.gas.to_string(),
        transaction.gas_price.to_string(),
        transaction.input.clone(),
        transaction.block_timestamp.to_string(),
        transaction.max_fee_per_gas.map(|num| num.to_string()).unwrap_or_default(),
        transaction.max_priority_fee_per_gas.map(|num| num.to_string()).unwrap_or_default(),
        transaction.transaction_type.map(|num| num.to_string()).unwrap_or_default(),
        transaction.max_fee_per_blob_gas.map(|num| num.to_string()).unwrap_or_default(),
        transaction.blob_versioned_hashes.join(","),
    ]
}