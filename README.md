# BioEnv: Biometric-Gated Local Secret Injector

![CI Status](https://github.com/AbdelrahmanMostafa-Eng/bioenv/workflows/CI/badge.svg)
![Release Status](https://github.com/AbdelrahmanMostafa-Eng/bioenv/workflows/Release/badge.svg)
![License](https://img.shields.io/github/license/AbdelrahmanMostafa-Eng/bioenv)
![Crates.io](https://img.shields.io/crates/v/bioenv)

## 🔒 Secure Your Secrets. Authenticate with Biometrics. Run Your Apps.

`BioEnv` is a cutting-edge Command Line Interface (CLI) tool designed to revolutionize how developers manage sensitive environment variables. Instead of relying on insecure `.env` files that are prone to accidental leaks, `BioEnv` leverages your operating system's native credential manager (Keychain on macOS, Credential Locker on Windows, Secret Service on Linux) to store secrets securely, protected by your biometric authentication (e.g., TouchID, Windows Hello) or a secure password fallback.

This project addresses a critical pain point in modern development workflows: the constant risk of exposing API keys, database credentials, and other sensitive information. `BioEnv` ensures that your secrets are never written to disk in plain text and are only injected into your application's process memory after a successful, secure authentication.

## ✨ Features

- **Biometric Protection:** Authenticate with your OS's native biometric systems (or a secure password) before accessing or injecting secrets.
- **Project Isolation:** Secrets are automatically scoped to your current project directory, preventing cross-project leaks.
- **Zero-Trust Injection:** Secrets are injected directly into the child process's environment variables, never touching the disk.
- **Memory Zeroization:** Sensitive data is actively wiped from `BioEnv`'s memory immediately after use.
- **`.env` File Import:** Seamlessly import existing `.env` files into your secure keychain, with an option to securely delete the original file.
- **Cross-Platform:** Built with Rust for native performance and compatibility across macOS, Windows, and Linux.
- **Developer Experience (DX):** Simple CLI commands for setting, getting, listing, deleting, and running applications with secrets.

## 🚀 Getting Started

### Installation

**From Crates.io (Recommended):**

```bash
cargo install bioenv
```

**From Source:**

```bash
git clone https://github.com/AbdelrahmanMostafa-Eng/bioenv.git
cd bioenv
cargo install --path .
```

### Usage

#### `bioenv set <KEY> [VALUE]`
Securely stores a key-value pair. If `VALUE` is omitted, you will be prompted to enter it securely.

```bash
bioenv set DATABASE_URL
# Enter value for DATABASE_URL: **********************************

bioenv set API_KEY my_super_secret_key
```

#### `bioenv get <KEY>`
Retrieves and displays a secret's value after biometric/password authentication.

```bash
bioenv get DATABASE_URL
# Confirm identity to view secret
# DATABASE_URL: postgres://user:pass@host:port/db
```

#### `bioenv list`
Lists all secret keys stored for the current project.

```bash
bioenv list
# Secrets for current project:
#  - DATABASE_URL
#  - API_KEY
```

#### `bioenv delete <KEY>`
Removes a secret from the keychain.

```bash
bioenv delete API_KEY
```

#### `bioenv import <FILE>`
Imports all key-value pairs from a `.env` file into the secure storage. You'll be prompted to securely delete the original file.

```bash
bioenv import .env.production
```

#### `bioenv run [--isolated] -- <COMMAND>`
Runs a command with all project-scoped secrets injected as environment variables. Requires authentication.

- `--isolated`: (Optional) Clears all existing environment variables from the parent process before injecting secrets, ensuring a clean and predictable environment.

```bash
bioenv run -- npm start
# Confirm identity to inject secrets
# (Your npm application starts with secrets injected)

bioenv run --isolated -- python manage.py runserver
```

## 📐 Architecture

`BioEnv` is designed with a modular, security-first architecture. Below is a high-level overview:

```mermaid
graph TD
    A[User] -->|Executes bioenv CLI| B(bioenv CLI)
    B -->|Parses Commands| C{CLI Commands}

    C -->|Set/Get/List/Delete| D[KeychainManager]
    D -->|OS Credential Manager API| E(OS Keychain/Credential Locker)

    C -->|Import .env| F[Importer]
    F -->|Reads .env file| G(Local .env file)
    F -->|Stores secrets| D

    C -->|Run Command| H[Security Module]
    H -->|Biometric/Password Prompt| I(User Authentication)
    I -->|Success| J[ProcessRunner]
    J -->|Fetches secrets| D
    J -->|Spawns Child Process| K(Your Application)
    K -->|Uses Injected Env Vars| L(Application Logic)

    subgraph Security Flow
        H -- Zeroize secrets --> J
        J -- Zeroize secrets --> K
    end

    subgraph Project Isolation
        B -- Hashes CWD --> D
    end
```

## 🔐 Security Model

`BioEnv` prioritizes security at every layer:

1.  **OS-Native Storage:** Secrets are stored using the operating system's built-in, highly secure credential management systems. These systems are designed to protect sensitive data from unauthorized access and are often backed by hardware-level encryption.
2.  **Biometric/Password Authentication:** Access to secrets (for viewing or injection) is gated by a mandatory authentication step. This ensures that only the authorized user can interact with the stored secrets.
3.  **Project-Scoped Isolation:** Each project's secrets are stored in a unique namespace derived from its directory path. This prevents secrets from one project from being accidentally exposed or used in another.
4.  **No Plaintext on Disk:** Secrets are never stored in plaintext files within your project directory. Once imported or set, they reside solely within the secure OS credential store.
5.  **In-Memory Injection:** When running a command, `BioEnv` retrieves secrets from the OS store and injects them directly into the child process's environment variables. They are not written to temporary files or passed as command-line arguments.
6.  **Memory Zeroization:** The `zeroize` crate is used to overwrite sensitive data in `BioEnv`'s memory with zeros immediately after it's no longer needed. This mitigates the risk of secrets being recovered from memory dumps.
7.  **Minimal Permissions:** `BioEnv` operates with the minimal necessary permissions. It does not require root access for its core functionality.

## 🤝 Contributing

We welcome contributions to `BioEnv`! Please see [CONTRIBUTING.md](https://github.com/AbdelrahmanMostafa-Eng/bioenv/blob/main/CONTRIBUTING.md) for guidelines.

## 📄 License

`BioEnv` is licensed under the MIT License. See [LICENSE](https://github.com/AbdelrahmanMostafa-Eng/bioenv/blob/main/LICENSE) for more information.

## 📞 Support & Community

For questions, bug reports, or feature requests, please open an issue on the [GitHub repository](https://github.com/AbdelrahmanMostafa-Eng/bioenv/issues).
