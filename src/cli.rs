use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "Ethereum ETL",
    version = "0.1.0",
    author = "Your Name <your.email@example.com>",
    about = "Ethereum ETL CLI"
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(
        name = "export_blocks_and_transactions",
        about = "Exports blocks and transactions"
    )]
    ExportBlocksAndTransactions {
        #[clap(long, value_parser, default_value = "0")]
        start_block: u64,
        #[clap(long, value_parser)]
        end_block: u64,
        #[clap(long, value_parser, default_value = "100")]
        batch_size: u64,
        #[clap(long, value_parser)]
        provider_uri: String,
        #[clap(long, value_parser, default_value = "5")]
        max_workers: usize,
        #[clap(long, value_parser)]
        blocks_output: Option<String>,
        #[clap(long, value_parser)]
        transactions_output: Option<String>,
    },
}