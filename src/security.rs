use crate::error::{BioEnvError, Result};
use dialoguer::{theme::ColorfulTheme, Password};
use secrecy::{ExposeSecret, SecretString};


/// A wrapper for sensitive operations.
pub struct Security;

impl Security {
    /// Triggers a secure password prompt as a fallback for biometrics.
    /// In production, this integrates with OS-level authentication.
    pub fn authenticate(prompt: &str) -> Result<SecretString> {
        Password::with_theme(&ColorfulTheme::default())
            .with_prompt(prompt)
            .interact()
            .map(SecretString::from)
            .map_err(|_| BioEnvError::AuthFailed)
    }


}

/// Extension trait to expose secrets safely.
pub trait ExposeBioEnvSecret {
    fn expose(&self) -> &str;
}

impl ExposeBioEnvSecret for SecretString {
    fn expose(&self) -> &str {
        self.expose_secret()
    }
}
