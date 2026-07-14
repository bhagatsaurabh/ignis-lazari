#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum InstanceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Unknown,
}
