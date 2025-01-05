use crate::domain::{Block, Transaction};
use crate::exporters::CsvExporter;
use crate::mappers::{block_to_csv_row, ethers_block_to_block, transaction_to_csv_row};
use crate::utils::ProgressTracker;
use anyhow::{Context, Result};
use ethers::providers::Middleware;
use ethers::types::BlockNumber;
use futures::future::join_all;
use std::convert::TryFrom;
use std::time::Instant;
use tracing::{info, instrument};

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

    #[instrument(skip_all, fields(start_block = %self.start_block, end_block = %self.end_block))]
    pub async fn run(&self) -> Result<()> {
        let start_time = Instant::now();

        let total_blocks = self.end_block - self.start_block + 1;
        let mut progress = ProgressTracker::new("export_blocks_and_transactions", Some(total_blocks));
        progress.start();

        let block_range = self.start_block..=self.end_block;
        let mut futures = vec![];
        let mut blocks_count = 0;
        let mut transactions_count = 0;

        for block_number in block_range {
            let provider = self.provider.clone();
            let exporter = self.exporter.clone();

            futures.push(tokio::spawn(async move {
                let block = provider
                    .get_block_with_txs(block_number)
                    .await
                    .map_err(|e| {
                        tracing::error!("Error getting block: {}", e);
                        e
                    })?
                    .context("Block not found")?;

                let block = ethers_block_to_block(block).unwrap();

                let block_csv_row = block_to_csv_row(&block);
                let transactions_csv_rows = block.transactions
                    .iter()
                    .map(transaction_to_csv_row)
                    .collect::<Vec<Vec<String>>>();

                let tx_count = block.transactions.len();

                let block_result = exporter.export_blocks_and_transactions(vec![block_csv_row], transactions_csv_rows);
                if let Err(e) = block_result {
                    tracing::error!("Error exporting block and transactions: {}", e);
                }

                Ok::<_, anyhow::Error>(tx_count)
            }));

            if futures.len() >= self.batch_size as usize {
                let results = join_all(futures.drain(..)).await;
                for result in results {
                    let tx_count = result??;
                    transactions_count += tx_count;
                }
                blocks_count += self.batch_size;
                progress.track(self.batch_size);
            }
        }

        if !futures.is_empty() {
            let results = join_all(futures).await;
            for result in results {
                let tx_count = result??;
                transactions_count += tx_count;
            }
            blocks_count += 1;
            progress.track(1);
        }

        let duration = start_time.elapsed();
        info!("Finished work. Total blocks processed: {}. Took: {:?}", blocks_count, duration);

        Ok(())
    }
}