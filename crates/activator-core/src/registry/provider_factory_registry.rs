use std::collections::HashMap;
use std::sync::Arc;

use plugin_api::ProviderFactory;

use super::RegistryError;

pub struct ProviderFactoryRegistry {
    factories: HashMap<String, Arc<dyn ProviderFactory>>,
}

impl ProviderFactoryRegistry {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, factory: Arc<dyn ProviderFactory>) {
        self.factories
            .insert(factory.provider_type().to_string(), factory);
    }

    pub fn get(&self, provider_type: &str) -> Result<Arc<dyn ProviderFactory>, RegistryError> {
        self.factories
            .get(provider_type)
            .cloned()
            .ok_or_else(|| RegistryError::UnknownProviderType(provider_type.to_owned()))
    }
}
