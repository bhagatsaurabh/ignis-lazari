use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OciInstanceResponse {
    pub data: OciInstanceData,
}

#[derive(Debug, Deserialize)]
pub struct OciInstanceData {
    #[serde(rename = "lifecycle-state")]
    pub lifecycle_state: String,
}

impl OciInstanceResponse {
    pub fn to_instance_state(&self) -> plugin_api::InstanceState {
        match self.data.lifecycle_state.as_str() {
            "RUNNING" => plugin_api::InstanceState::Running,
            "STOPPED" => plugin_api::InstanceState::Stopped,
            "STARTING" => plugin_api::InstanceState::Starting,
            "STOPPING" => plugin_api::InstanceState::Stopping,
            _ => plugin_api::InstanceState::Unknown,
        }
    }
}
