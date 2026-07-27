use crate::config::Config;
use crate::error::{BioEnvError, Result};
use crate::security::ExposeBioEnvSecret;
use secrecy::SecretString;
use keyring::Entry;

/// Professional-grade Keychain management.
pub struct KeychainManager;

impl KeychainManager {
    const METADATA_KEY: &'static str = "__bioenv_metadata__";

    /// Persists a secret in the OS secure enclave.
    pub fn store(key: &str, value: &SecretString) -> Result<()> {
        let namespace = Config::project_namespace()?;
        let entry = Entry::new(&namespace, key)?;
        entry.set_password(value.expose())?;
        
        // Track the key in metadata for listing
        Self::update_metadata(key, true)?;
        Ok(())
    }

    /// Retrieves a secret from the OS secure enclave.
    pub fn retrieve(key: &str) -> Result<SecretString> {
        let namespace = Config::project_namespace()?;
        let entry = Entry::new(&namespace, key)?;
        
        entry.get_password()
            .map(SecretString::from)
            .map_err(|e| match e {
                keyring::Error::NoEntry => BioEnvError::SecretNotFound(key.to_string()),
                _ => BioEnvError::Keychain(e),
            })
    }

    /// Removes a secret and its metadata entry.
    pub fn remove(key: &str) -> Result<()> {
        let namespace = Config::project_namespace()?;
        let entry = Entry::new(&namespace, key)?;
        
        entry.delete_password().map_err(|e| match e {
            keyring::Error::NoEntry => BioEnvError::SecretNotFound(key.to_string()),
            _ => BioEnvError::Keychain(e),
        })?;
        
        Self::update_metadata(key, false)?;
        Ok(())
    }

    /// Lists all keys scoped to the current project.
    pub fn list() -> Result<Vec<String>> {
        let namespace = Config::project_namespace()?;
        let entry = Entry::new(&namespace, Self::METADATA_KEY)?;
        
        match entry.get_password() {
            Ok(json) => Ok(serde_json::from_str(&json)?),
            Err(_) => Ok(Vec::new()),
        }
    }

    fn update_metadata(key: &str, add: bool) -> Result<()> {
        let mut keys = Self::list()?;
        if add {
            if !keys.contains(&key.to_string()) {
                keys.push(key.to_string());
            }
        } else {
            keys.retain(|k| k != key);
        }
        
        let namespace = Config::project_namespace()?;
        let entry = Entry::new(&namespace, Self::METADATA_KEY)?;
        let json = serde_json::to_string(&keys)?;
        entry.set_password(&json)?;
        Ok(())
    }
}
