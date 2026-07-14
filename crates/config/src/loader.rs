use crate::error::ConfigError;
use crate::models::Config;
use crate::validator::ConfigValidator;

pub struct ConfigLoader;

impl ConfigLoader {
    pub fn load(path: &str) -> Result<Config, ConfigError> {
        let contents = std::fs::read_to_string(path)?;

        let config = serde_norway::from_str(&contents)?;

        ConfigValidator::validate(&config)?;

        Ok(config)
    }
}
