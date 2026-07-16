use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecError {
    #[error("failed to spawn process '{0}': {1}")]
    Spawn(String, std::io::Error),

    #[error("process '{0}' timed out after {1:?}")]
    Timeout(String, std::time::Duration),

    #[error("process '{0}' exited with status {1}: {2}")]
    NonZeroExit(String, i32, String),

    #[error("failed to parse output of '{0}' as JSON: {1}")]
    JsonParse(String, serde_json::Error),
}
