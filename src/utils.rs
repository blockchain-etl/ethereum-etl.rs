use thiserror::Error;

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