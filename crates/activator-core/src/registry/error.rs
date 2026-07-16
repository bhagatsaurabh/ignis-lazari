use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("instance '{0}' is already registered")]
    DuplicateInstance(String),

    #[error("instance '{0}' was not found")]
    InstanceNotFound(String),

    #[error("no provider factory registered for type '{0}'")]
    UnknownProviderType(String),
}
