mod cli;
mod config;
mod error;
mod import;
mod keychain;
mod process;
mod security;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use keychain::KeychainManager;
use process::ProcessRunner;
use security::{ExposeBioEnvSecret, Security};
use secrecy::SecretString;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize professional logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_env("BIOENV_LOG"))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Set { key, value } => {
            let secret = match value {
                Some(v) => SecretString::from(v),
                None => Security::authenticate(&format!("Enter value for '{}':", key))?,
            };
            KeychainManager::store(&key, &secret)?;
            println!("✅ Secret '{}' stored securely.", key);
        }
        Commands::Get { key } => {
            Security::authenticate("Verify identity to view secret")?;
            let secret = KeychainManager::retrieve(&key)?;
            println!("🔑 {}: {}", key, secret.expose());
        }
        Commands::List => {
            let keys = KeychainManager::list()?;
            if keys.is_empty() {
                println!("ℹ️  No secrets found for this project.");
            } else {
                println!("📋 Secrets for current project:");
                for key in keys {
                    println!("  - {}", key);
                }
            }
        }
        Commands::Delete { key } => {
            KeychainManager::remove(&key)?;
            println!("✅ Secret '{}' removed.", key);
        }
        Commands::Import { path } => {
            import::Importer::from_env_file(path)?;
        }
        Commands::Run { isolated, command } => {
            Security::authenticate("Verify identity to inject secrets")?;
            
            let cmd_name = &command[0];
            let args = &command[1..];
            
            let exit_code = ProcessRunner::execute(cmd_name, args, isolated)?;
            std::process::exit(exit_code);
        }
    }

    Ok(())
}
