use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OciConfig {
    pub instance_id: String,

    #[serde(default)]
    pub profile: Option<String>,

    #[serde(default = "default_cli_binary")]
    pub cli_binary: String,
}

fn default_cli_binary() -> String {
    "oci".to_string()
}

impl OciConfig {
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let contents = std::fs::read_to_string(path)
            .map_err(|err| format!("failed to read '{}': {err}", path.display()))?;

        serde_norway::from_str(&contents)
            .map_err(|err| format!("failed to parse '{}': {err}", path.display()))
    }
}
