mod domain;
mod mappers;
mod providers;
mod jobs;
mod exporters;
mod utils;

use clap::Parser;
use jobs::ExportBlocksJob;
use providers::get_provider_from_uri;
use exporters::CsvExporter;
use std::path::PathBuf;
use tokio;

#[derive(Parser, Debug)]
#[clap(
    name = "Ethereum ETL",
    version = "0.1.0",
    author = "Your Name <your.email@example.com>",
    about = "Ethereum ETL CLI"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(name = "export_blocks_and_transactions", about = "Export blocks and transactions", alias = "export-blocks-and-transactions")]
    ExportBlocksAndTransactions(ExportOpts),
}

#[derive(Parser, Debug)]
struct ExportOpts {
    #[clap(long, help = "Start block")]
    start_block: u64,

    #[clap(long, help = "End block")]
    end_block: u64,

    #[clap(long, default_value = "100", help = "Batch size")]
    batch_size: u64,

    #[clap(long, help = "Provider URI")]
    provider_uri: String,

    #[clap(long, default_value = "5", help = "Max workers")]
    max_workers: usize,

    #[clap(long, help = "Output directory for blocks")]
    blocks_output: Option<PathBuf>,

    #[clap(long, help = "Output directory for transactions")]
    transactions_output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::ExportBlocksAndTransactions(export_opts) => {
            let provider = get_provider_from_uri(&export_opts.provider_uri).await?;
            let exporter = CsvExporter::new(
                export_opts.blocks_output,
                export_opts.transactions_output,
            )?;
            let job = ExportBlocksJob::new(
                export_opts.start_block,
                export_opts.end_block,
                export_opts.batch_size,
                provider,
                export_opts.max_workers,
                exporter,
            );
            job.run().await?;
        }
    }

    Ok(())
}