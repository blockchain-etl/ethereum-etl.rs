use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Block {
    pub number: u64,
    pub hash: String,
    pub parent_hash: String,
    pub nonce: String,
    pub sha3_uncles: String,
    pub logs_bloom: String,
    pub transactions_root: String,
    pub state_root: String,
    pub receipts_root: String,
    pub miner: String,
    pub difficulty: String,
    pub total_difficulty: Option<String>,
    pub size: u64,
    pub extra_data: String,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub transaction_count: u64,
    #[serde(default)]
    pub base_fee_per_gas: Option<u64>,
    #[serde(default)]
    pub withdrawals_root: Option<String>,
    #[serde(default)]
    pub blob_gas_used: Option<u64>,
    #[serde(default)]
    pub excess_blob_gas: Option<u64>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub hash: String,
    pub nonce: u64,
    pub block_hash: Option<String>,
    pub block_number: Option<u64>,
    pub transaction_index: Option<u64>,
    pub from_address: String,
    pub to_address: Option<String>,
    pub value: String,
    pub gas: u64,
    pub gas_price: String,
    pub input: String,
    pub block_timestamp: u64,
    #[serde(default)]
    pub max_fee_per_gas: Option<u64>,
    #[serde(default)]
    pub max_priority_fee_per_gas: Option<u64>,
    #[serde(default)]
    pub transaction_type: Option<u64>,
    #[serde(default)]
    pub max_fee_per_blob_gas: Option<u64>,
    #[serde(default)]
    pub blob_versioned_hashes: Vec<String>
}