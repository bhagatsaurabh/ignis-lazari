use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub instances: Vec<InstanceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct InstanceConfig {
    pub id: String,

    pub provider: String,

    #[serde(default)]
    pub allowed_origins: Vec<String>,

    pub provider_config: ProviderConfigRef,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ProviderConfigRef {
    File { path: String },
}
