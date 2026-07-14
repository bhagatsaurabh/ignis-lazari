use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProviderError {
    #[error("{0}")]
    Message(String),
}
