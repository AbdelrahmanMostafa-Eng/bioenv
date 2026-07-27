use clap::{Parser, Subcommand};

/// The G.O.A.T. CLI definition for BioEnv.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 🔒 Store a secret for the current project
    Set {
        /// The name/key of the secret
        key: String,
        /// The value (omitting this will trigger a secure prompt)
        value: Option<String>,
    },
    /// 🔑 View a secret's value (requires authentication)
    Get {
        /// The name of the secret to retrieve
        key: String,
    },
    /// 📋 List all secret keys for this project
    List,
    /// 🗑️  Delete a secret from the project
    Delete {
        /// The name of the secret to remove
        key: String,
    },
    /// 📥 Import secrets from a .env file
    Import {
        /// Path to the .env file
        #[arg(default_value = ".env")]
        path: String,
    },
    /// 🚀 Run a command with secrets injected
    Run {
        /// Clear existing environment variables (Pure Isolation)
        #[arg(short, long)]
        isolated: bool,
        /// The command and arguments to execute
        #[arg(last = true, required = true)]
        command: Vec<String>,
    },
}
