use std::time::Duration;

use serde::de::DeserializeOwned;
use tokio::process::Command;
use tokio::time::timeout;

use crate::ExecError;

pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

/// Generic async runner for shelling out to a provider's CLI.
/// Any provider without a Rust SDK builds on this instead of
/// hand-rolling process spawning.
pub struct CliRunner {
    binary: String,
    default_timeout: Duration,
}

impl CliRunner {
    pub fn new(binary: impl Into<String>) -> Self {
        Self {
            binary: binary.into(),
            default_timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    pub async fn run(&self, args: &[&str]) -> Result<ExecOutput, ExecError> {
        let mut command = Command::new(&self.binary);
        command.args(args);

        let output = timeout(self.default_timeout, command.output())
            .await
            .map_err(|_| ExecError::Timeout(self.binary.clone(), self.default_timeout))?
            .map_err(|err| ExecError::Spawn(self.binary.clone(), err))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(ExecError::NonZeroExit(
                self.binary.clone(),
                output.status.code().unwrap_or(-1),
                if stderr.is_empty() {
                    stdout.clone()
                } else {
                    stderr.clone()
                },
            ));
        }

        Ok(ExecOutput { stdout, stderr })
    }

    /// Convenience for CLIs that support `--output json` / similar.
    pub async fn run_json<T: DeserializeOwned>(&self, args: &[&str]) -> Result<T, ExecError> {
        let output = self.run(args).await?;

        serde_json::from_str(&output.stdout)
            .map_err(|err| ExecError::JsonParse(self.binary.clone(), err))
    }
}
