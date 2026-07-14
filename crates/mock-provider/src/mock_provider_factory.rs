use std::path::Path;
use std::sync::Arc;

use plugin_api::{Provider, ProviderError, ProviderFactory};

use crate::MockProvider;

pub struct MockProviderFactory;

impl ProviderFactory for MockProviderFactory {
    fn provider_type(&self) -> &'static str {
        "mock"
    }

    fn create(&self, _config_path: Option<&Path>) -> Result<Arc<dyn Provider>, ProviderError> {
        Ok(Arc::new(MockProvider::new()))
    }
}
