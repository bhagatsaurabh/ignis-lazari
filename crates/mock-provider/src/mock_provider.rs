use async_trait::async_trait;
use plugin_api::{InstanceState, Provider, ProviderError};

pub struct MockProvider;

impl MockProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Provider for MockProvider {
    fn provider_type(&self) -> &'static str {
        "mock"
    }

    async fn start(&self) -> Result<(), ProviderError> {
        println!("Starting mock instance...");
        Ok(())
    }

    async fn stop(&self) -> Result<(), ProviderError> {
        println!("Stopping mock instance...");
        Ok(())
    }

    async fn status(&self) -> Result<InstanceState, ProviderError> {
        Ok(InstanceState::Running)
    }
}
