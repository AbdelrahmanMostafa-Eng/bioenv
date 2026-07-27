use crate::error::Result;
use sha2::{Digest, Sha256};
use std::env;

/// Core configuration and project isolation logic.
pub struct Config;

impl Config {
    /// The global application identifier.
    pub const APP_NAME: &'static str = "bioenv";

    /// Generates a deterministic, project-specific namespace.
    /// This ensures secrets are isolated to the directory they were created in.
    pub fn project_namespace() -> Result<String> {
        let current_dir = env::current_dir()?;
        let path_bytes = current_dir.to_string_lossy().as_bytes().to_vec();
        
        let mut hasher = Sha256::new();
        hasher.update(&path_bytes);
        let hash = hasher.finalize();
        
        // Use hex encoding for a clean, URL-safe namespace string
        Ok(format!("{}-{}", Self::APP_NAME, hex::encode(&hash[..8])))
    }
}
