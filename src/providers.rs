use ethers::providers::{Http, Middleware, Provider};
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum ProviderError {
    #[error("Invalid provider URI")]
    InvalidUri,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub async fn get_provider_from_uri(uri: &str) -> Result<Provider<Http>, ProviderError> {
    let parsed_uri = Url::parse(uri).map_err(|_| ProviderError::InvalidUri)?;

    match parsed_uri.scheme() {
        "http" | "https" => {
            let provider = Provider::<Http>::try_from(parsed_uri.as_str())
                .map_err(|e| ProviderError::Other(anyhow::anyhow!("Failed to create HTTP provider: {}", e)))?;
            Ok(provider)
        }
        _ => Err(ProviderError::InvalidUri),
    }
}