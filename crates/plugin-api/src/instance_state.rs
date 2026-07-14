#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstanceState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Unknown,
}
