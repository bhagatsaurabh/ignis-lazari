use std::path::Path;
use std::sync::Arc;

use plugin_api::{Provider, ProviderError, ProviderFactory};

use crate::config::OciConfig;
use crate::oci_provider::OciProvider;

pub struct OciProviderFactory;

impl ProviderFactory for OciProviderFactory {
    fn provider_type(&self) -> &'static str {
        "oracle-oci"
    }

    fn create(&self, config_path: Option<&Path>) -> Result<Arc<dyn Provider>, ProviderError> {
        let path = config_path.ok_or_else(|| {
            ProviderError::Message("oracle-oci provider requires provider_config".into())
        })?;

        let config = OciConfig::load(path).map_err(ProviderError::Message)?;

        Ok(Arc::new(OciProvider::new(config)))
    }
}
