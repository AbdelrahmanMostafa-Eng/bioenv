use crate::error::{BioEnvError, Result};
use crate::keychain::KeychainManager;
use crate::security::ExposeBioEnvSecret;
use std::process::{Command, Stdio};

/// High-performance process runner with environment injection.
pub struct ProcessRunner;

impl ProcessRunner {
    /// Executes a command with project secrets injected.
    pub fn execute(command: &str, args: &[String], isolated: bool) -> Result<i32> {
        let keys = KeychainManager::list()?;
        
        let mut cmd = Command::new(command);
        cmd.args(args);

        if isolated {
            cmd.env_clear();
        }

        // Inject secrets directly into process environment
        for key in keys {
            let secret = KeychainManager::retrieve(&key)?;
            cmd.env(key, secret.expose());
        }

        let mut child = cmd
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|e| BioEnvError::ProcessExecution(e.to_string()))?;

        let status = child.wait()?;
        
        Ok(status.code().unwrap_or(0))
    }
}
