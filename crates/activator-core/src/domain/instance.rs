use plugin_api::InstanceState;

#[derive(Debug, Clone)]
pub struct ManagedInstance {
    id: String,
    provider: String,
    state: InstanceState,
}

impl ManagedInstance {
    pub fn new(id: String, provider: String) -> Self {
        Self {
            id,
            provider,
            state: InstanceState::Stopped,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn provider(&self) -> &str {
        &self.provider
    }

    pub fn state(&self) -> InstanceState {
        self.state
    }

    pub fn mark_starting(&mut self) {
        self.state = InstanceState::Starting;
    }

    pub fn mark_running(&mut self) {
        self.state = InstanceState::Running;
    }

    pub fn mark_stopping(&mut self) {
        self.state = InstanceState::Stopping;
    }

    pub fn mark_stopped(&mut self) {
        self.state = InstanceState::Stopped;
    }
}
