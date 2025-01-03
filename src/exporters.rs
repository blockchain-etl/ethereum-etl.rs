use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use csv::Writer;
use crate::domain::{Block, Transaction};
use crate::mappers::{block_to_csv_row, transaction_to_csv_row};

#[derive(Clone)]
pub struct CsvExporter {
    blocks_writer: Arc<Mutex<Option<Writer<File>>>>,
    transactions_writer: Arc<Mutex<Option<Writer<File>>>>,
}

impl CsvExporter {
    pub fn new(
        blocks_output: Option<PathBuf>,
        transactions_output: Option<PathBuf>,
    ) -> Result<Self, std::io::Error> {
        let blocks_writer = match blocks_output {
            Some(path) => {
                let mut writer = csv::Writer::from_path(path)?;
                writer.write_record(&[
                    "number",
                    "hash",
                    "parent_hash",
                    "nonce",
                    "sha3_uncles",
                    "logs_bloom",
                    "transactions_root",
                    "state_root",
                    "receipts_root",
                    "miner",
                    "difficulty",
                    "total_difficulty",
                    "size",
                    "extra_data",
                    "gas_limit",
                    "gas_used",
                    "timestamp",
                    "transaction_count",
                    "base_fee_per_gas",
                    "withdrawals_root",
                    "blob_gas_used",
                    "excess_blob_gas"
                ])?;
                Some(writer)
            }
            None => None,
        };

        let transactions_writer = match transactions_output {
            Some(path) => {
                let mut writer = csv::Writer::from_path(path)?;
                writer.write_record(&[
                    "hash",
                    "nonce",
                    "block_hash",
                    "block_number",
                    "transaction_index",
                    "from_address",
                    "to_address",
                    "value",
                    "gas",
                    "gas_price",
                    "input",
                    "block_timestamp",
                    "max_fee_per_gas",
                    "max_priority_fee_per_gas",
                    "transaction_type",
                    "max_fee_per_blob_gas",
                    "blob_versioned_hashes"
                ])?;
                Some(writer)
            }
            None => None,
        };

        Ok(CsvExporter {
            blocks_writer: Arc::new(Mutex::new(blocks_writer)),
            transactions_writer: Arc::new(Mutex::new(transactions_writer)),
        })
    }

    pub fn export_blocks_and_transactions(
        &self,
        blocks: Vec<Vec<String>>,
        transactions: Vec<Vec<String>>,
    ) -> Result<(), std::io::Error> {
        if let Some(mut writer) = self.blocks_writer.lock().unwrap().as_mut() {
            for block in blocks {
                writer.write_record(&block)?;
            }
            writer.flush()?;
        }

        if let Some(mut writer) = self.transactions_writer.lock().unwrap().as_mut() {
            for transaction in transactions {
                writer.write_record(&transaction)?;
            }
            writer.flush()?;
        }

        Ok(())
    }
}