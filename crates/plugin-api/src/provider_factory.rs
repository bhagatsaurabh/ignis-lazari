use std::path::Path;
use std::sync::Arc;

use crate::{Provider, ProviderError};

pub trait ProviderFactory: Send + Sync {
    fn provider_type(&self) -> &'static str;

    fn create(&self, config_path: Option<&Path>) -> Result<Arc<dyn Provider>, ProviderError>;
}
