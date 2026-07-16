use std::path::Path;

use crate::error::ConfigError;
use crate::models::Config;
use crate::validator::ConfigValidator;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load(config_dir: &Path) -> Result<Config, ConfigError> {
        let config_path = config_dir.join("config.yaml");

        let contents = std::fs::read_to_string(&config_path).map_err(ConfigError::Io)?;

        let config = serde_norway::from_str(&contents)?;

        ConfigValidator::validate(&config)?;

        Ok(config)
    }
}
