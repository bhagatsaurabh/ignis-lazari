use crate::Config;
use crate::error::ConfigError;
use std::collections::HashSet;

pub struct ConfigValidator;

impl ConfigValidator {
    pub fn validate(config: &Config) -> Result<(), ConfigError> {
        Self::validate_server(config)?;
        Self::validate_instances(config)?;

        Ok(())
    }

    fn validate_server(config: &Config) -> Result<(), ConfigError> {
        if config.server.port == 0 {
            return Err(ConfigError::Validation(
                "server.port must be greater than 0".into(),
            ));
        }

        Ok(())
    }

    fn validate_instances(config: &Config) -> Result<(), ConfigError> {
        if config.instances.is_empty() {
            return Err(ConfigError::Validation(
                "at least one instance must be configured".into(),
            ));
        }

        let mut ids = HashSet::new();
        for instance in &config.instances {
            if !ids.insert(instance.id.clone()) {
                return Err(ConfigError::Validation(format!(
                    "duplicate instance id '{}'",
                    instance.id
                )));
            }
        }

        Ok(())
    }
}
