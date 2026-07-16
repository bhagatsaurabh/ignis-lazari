use async_trait::async_trait;
use plugin_api::{InstanceState, Provider, ProviderError};
use process_exec::CliRunner;

use crate::config::OciConfig;
use crate::response::OciInstanceResponse;

pub struct OciProvider {
    runner: CliRunner,
    config: OciConfig,
}

impl OciProvider {
    pub fn new(config: OciConfig) -> Self {
        let runner = CliRunner::new(config.cli_binary.clone());
        Self { runner, config }
    }

    fn base_args(&self) -> Vec<&str> {
        let mut args = vec!["--instance-id", self.config.instance_id.as_str()];

        if let Some(profile) = &self.config.profile {
            args.push("--profile");
            args.push(profile.as_str());
        }

        args
    }

    async fn run_action(&self, action: &str) -> Result<InstanceState, ProviderError> {
        tracing::info!(instance_id = %self.config.instance_id, action, "running oci action");

        let mut args = vec!["compute", "instance", "action"];
        args.extend(self.base_args());
        args.push("--action");
        args.push(action);

        let output = self
            .runner
            .run(&args)
            .await
            .map_err(|err| ProviderError::Message(err.to_string()))?;

        let parsed: OciInstanceResponse = serde_json::from_str(&output.stdout)
            .map_err(|err| ProviderError::Message(format!("failed to parse oci output: {err}")))?;

        Ok(parsed.to_instance_state())
    }
}

#[async_trait]
impl Provider for OciProvider {
    fn provider_type(&self) -> &'static str {
        "oracle-oci"
    }

    async fn start(&self) -> Result<(), ProviderError> {
        self.run_action("START").await?;
        Ok(())
    }

    async fn stop(&self) -> Result<(), ProviderError> {
        self.run_action("STOP").await?;
        Ok(())
    }

    async fn status(&self) -> Result<InstanceState, ProviderError> {
        tracing::debug!(instance_id = %self.config.instance_id, "checking oci status");

        let mut args = vec!["compute", "instance", "get"];
        args.extend(self.base_args());

        let response: OciInstanceResponse = self
            .runner
            .run_json(&args)
            .await
            .map_err(|err| ProviderError::Message(err.to_string()))?;

        Ok(response.to_instance_state())
    }
}
