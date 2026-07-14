use async_trait::async_trait;

use crate::{InstanceState, ProviderError};

#[async_trait]
pub trait Provider: Send + Sync {
    fn provider_type(&self) -> &'static str;

    async fn start(&self) -> Result<(), ProviderError>;

    async fn stop(&self) -> Result<(), ProviderError>;

    async fn status(&self) -> Result<InstanceState, ProviderError>;
}
