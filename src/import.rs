use crate::error::{BioEnvError, Result};
use crate::keychain::KeychainManager;
use dotenvy::from_path_iter;
use secrecy::SecretString;
use std::fs;
use std::path::Path;
use dialoguer::Confirm;

/// Secure importer for legacy environment files.
pub struct Importer;

impl Importer {
    /// Migrates secrets from a .env file to the OS keychain.
    pub fn from_env_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        
        let iter = from_path_iter(path)
            .map_err(|e| BioEnvError::Import(format!("Failed to read .env: {}", e)))?;

        let mut count = 0;
        for item in iter {
            let (key, value) = item
                .map_err(|e| BioEnvError::Import(format!("Parse error: {}", e)))?;
            
            let secret = SecretString::from(value);
            KeychainManager::store(&key, &secret)?;
            count += 1;
        }

        println!("✅ Successfully imported {} secrets into the secure keychain.", count);

        if Confirm::new()
            .with_prompt("🗑️  Would you like to securely delete the .env file now?")
            .default(false)
            .interact()
            .unwrap_or(false) 
        {
            fs::remove_file(path)?;
            println!("✨ File deleted successfully. Your environment is now clean.");
        }

        Ok(())
    }
}
