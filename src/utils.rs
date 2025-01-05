use thiserror::Error;
use std::time::Instant;
use tracing::info;

pub struct ProgressTracker {
    name: String,
    total_items: Option<u64>,
    start_time: Option<Instant>,
    processed_items: u64,
    log_percentage_step: u64,
    log_items_step: u64,
    last_logged_percentage: u64,
    last_logged_items: u64,
}

impl ProgressTracker {
    pub fn new(name: &str, total_items: Option<u64>) -> Self {
        Self {
            name: name.to_string(),
            total_items,
            start_time: None,
            processed_items: 0,
            log_percentage_step: 10,
            log_items_step: 5000,
            last_logged_percentage: 0,
            last_logged_items: 0,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        info!("Started {}. Items to process: {}", self.name, self.total_items_display());
    }

    pub fn track(&mut self, item_count: u64) {
        self.processed_items += item_count;

        if let Some(total_items) = self.total_items {
            let percentage = (self.processed_items as f64 / total_items as f64 * 100.0) as u64;
            if percentage - self.last_logged_percentage >= self.log_percentage_step {
                info!(
                    "{} items processed. Progress is {}%",
                    self.processed_items, percentage
                );
                self.last_logged_percentage = percentage;
            }
        } else if self.processed_items - self.last_logged_items >= self.log_items_step {
            info!("{} items processed.", self.processed_items);
            self.last_logged_items = self.processed_items;
        }
    }

    pub fn finish(&self) {
        let elapsed = self.start_time.map(|t| t.elapsed());
        info!(
            "Finished {}. Total items processed: {}. Took: {:?}",
            self.name,
            self.processed_items,
            elapsed
        );
    }

    fn total_items_display(&self) -> String {
        match self.total_items {
            Some(total) => total.to_string(),
            None => "Unknown".to_string(),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ProviderError(#[from] crate::providers::ProviderError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    CsvError(#[from] csv::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}