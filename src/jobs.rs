use crate::domain::{Block, Transaction};
use crate::exporters::CsvExporter;
use crate::mappers::{block_to_csv_row, ethers_block_to_block, transaction_to_csv_row};
use anyhow::{Context, Result};
use ethers::providers::Middleware;
use ethers::types::BlockNumber;
use futures::future::join_all;
use std::convert::TryFrom;
use tokio::time::{sleep, Duration};

pub struct ExportBlocksJob<M: Middleware> {
    start_block: u64,
    end_block: u64,
    batch_size: u64,
    provider: M,
    max_workers: usize,
    exporter: CsvExporter,
}

impl<M> ExportBlocksJob<M>
where
    M: Middleware + 'static + Clone,
    M::Error: 'static,
{
    pub fn new(
        start_block: u64,
        end_block: u64,
        batch_size: u64,
        provider: M,
        max_workers: usize,
        exporter: CsvExporter,
    ) -> Self {
        Self {
            start_block,
            end_block,
            batch_size,
            provider,
            max_workers,
            exporter,
        }
    }

    pub async fn run(&self) -> Result<()> {
        let block_range = self.start_block..=self.end_block;
        let mut futures = vec![];

        for block_number in block_range {
            let provider = self.provider.clone();
            let exporter = self.exporter.clone();

            futures.push(tokio::spawn(async move {
                let block = provider
                    .get_block_with_txs(block_number)
                    .await
                    .map_err(|e| {
                        eprintln!("Error getting block: {}", e);
                        e
                    })?
                    .context("Block not found")?;

                let block = ethers_block_to_block(block).unwrap();

                let block_csv_row = block_to_csv_row(&block);
                let transactions_csv_rows = block.transactions
                    .iter()
                    .map(transaction_to_csv_row)
                    .collect::<Vec<Vec<String>>>();

                exporter.export_blocks_and_transactions(vec![block_csv_row], transactions_csv_rows)?;

                Ok::<(), anyhow::Error>(())
            }));

            if futures.len() >= self.max_workers {
                let results = join_all(futures.drain(..)).await;
                for result in results {
                    result??;
                }
            }
        }

        let results = join_all(futures).await;
        for result in results {
            result??;
        }

        Ok(())
    }
}