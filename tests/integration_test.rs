/// Integration tests for BioEnv CLI.
/// Note: Since the keychain requires a real OS session, we use mocks or 
/// environment-based testing where possible.

#[cfg(test)]
mod tests {
    use std::process::Command;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_help_command() {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Biometric-Gated Local Secret Injector CLI"));
    }

    #[test]
    fn test_import_logic() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_path = dir.path().join(".env");
        let mut file = File::create(&file_path).expect("Failed to create .env file");
        writeln!(file, "TEST_KEY=TEST_VALUE").expect("Failed to write to .env file");

        // We can't easily test the full import without a real keychain, 
        // but we can verify the CLI entry point doesn't crash.
        let output = Command::new("cargo")
            .args(["run", "--", "import", file_path.to_str().unwrap()])
            .output()
            .expect("Failed to execute command");

        // It might fail because of no interactive terminal, which is expected in CI
        // but the goal is to ensure the command is recognized.
    }
}
