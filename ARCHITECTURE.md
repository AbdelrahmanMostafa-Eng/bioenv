# BioEnv Architecture & Security Model

This document outlines the high-level design, security primitives, and implementation details of `BioEnv`.

## 1. Project Isolation Logic

BioEnv uses a **Deterministic Namespace Hashing** strategy to ensure that secrets are isolated to the specific directory where they were defined.

1.  **Input:** The absolute path of the current working directory (`std::env::current_dir`).
2.  **Hashing:** The path is hashed using **SHA-256**.
3.  **Namespace:** The first 8 bytes of the hash are hex-encoded and prefixed with `bioenv-`.
    *   *Example:* `/home/user/projects/api` becomes `bioenv-a1b2c3d4`.

This prevents a developer from accidentally accessing "Project A" secrets while working in "Project B".

## 2. Security Primitives

### Memory Safety (`secrecy` & `zeroize`)
BioEnv handles all sensitive data (API keys, passwords) using the `SecretString` type from the `secrecy` crate.
*   **In-Memory Protection:** Secrets are wrapped in a type that prevents them from being accidentally logged or printed to the console (via `Debug` or `Display`).
*   **Zeroization:** When a `SecretString` is dropped, the underlying memory is explicitly overwritten with zeros using the `zeroize` crate. This mitigates the risk of secrets lingering in RAM where they could be recovered by memory-dump attacks.

### OS-Native Storage (`keyring`)
Secrets are never stored in plaintext on disk. BioEnv leverages the host operating system's native secure enclave:
*   **macOS:** Apple Keychain
*   **Windows:** Credential Locker
*   **Linux:** Secret Service API (libsecret/dbus)

These systems provide hardware-backed encryption (on supported devices) and OS-level access controls.

## 3. The Injection Pipeline

When the `run` command is executed:
1.  **Authentication:** The user is prompted for their OS-level biometric or password.
2.  **Retrieval:** BioEnv fetches the secrets for the current namespace from the OS Keychain.
3.  **Spawning:** A child process is spawned using `std::process::Command`.
4.  **Injection:** The secrets are passed into the child process's environment variables.
5.  **Zeroization:** The secrets are dropped from BioEnv's memory as soon as the child process starts.

## 4. Module Overview

| Module | Responsibility |
| :--- | :--- |
| `main.rs` | Entry point, CLI routing, and logging initialization. |
| `cli.rs` | CLI definition using `clap` (Commands: set, get, list, delete, import, run). |
| `security.rs` | Authentication gating and secret wrapping logic. |
| `keychain.rs` | High-level interface for OS Keychain operations. |
| `config.rs` | Directory hashing and project namespace generation. |
| `process.rs` | Child process spawning and environment variable injection. |
| `import.rs` | Logic for parsing `.env` files and migrating them to secure storage. |
| `error.rs` | Centralized, library-grade error handling. |
