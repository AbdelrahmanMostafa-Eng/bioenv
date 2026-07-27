use thiserror::Error;

/// The G.O.A.T. error handling for BioEnv.
/// Uses `thiserror` for precise, library-grade error categorization.
#[derive(Debug, Error)]
pub enum BioEnvError {
    #[error("Keychain access failed: {0}")]
    Keychain(#[from] keyring::Error),

    #[error("I/O operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to serialize/deserialize data: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Secret '{0}' was not found in the current project namespace")]
    SecretNotFound(String),

    #[error("Authentication failed or was cancelled by the user")]
    AuthFailed,

    #[error("Child process execution failed: {0}")]
    ProcessExecution(String),

    #[error("Environment import failed: {0}")]
    Import(String),

}

/// A specialized Result type for BioEnv operations.
pub type Result<T> = std::result::Result<T, BioEnvError>;
