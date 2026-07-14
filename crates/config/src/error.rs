use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read configuration file: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse configuration file: {0}")]
    Parse(#[from] serde_norway::Error),

    #[error("{0}")]
    Validation(String),
}
