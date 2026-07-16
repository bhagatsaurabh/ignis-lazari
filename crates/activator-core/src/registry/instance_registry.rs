use std::collections::HashMap;
use std::sync::Arc;

use plugin_api::Provider;

use super::RegistryError;

pub struct RegisteredInstance {
    pub provider: Arc<dyn Provider>,
    pub allowed_origins: Vec<String>,
}

pub struct InstanceRegistry {
    instances: HashMap<String, Arc<RegisteredInstance>>,
}

impl InstanceRegistry {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        instance_id: String,
        provider: Arc<dyn Provider>,
        allowed_origins: Vec<String>,
    ) -> Result<(), RegistryError> {
        if self.instances.contains_key(&instance_id) {
            return Err(RegistryError::DuplicateInstance(instance_id));
        }

        self.instances.insert(
            instance_id,
            Arc::new(RegisteredInstance {
                provider,
                allowed_origins,
            }),
        );

        Ok(())
    }

    pub fn get(&self, instance_id: &str) -> Result<Arc<RegisteredInstance>, RegistryError> {
        self.instances
            .get(instance_id)
            .cloned()
            .ok_or_else(|| RegistryError::InstanceNotFound(instance_id.to_owned()))
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }
}
