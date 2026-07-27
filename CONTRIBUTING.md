# Contributing to BioEnv

We welcome contributions to `BioEnv`! Whether it's bug reports, feature requests, documentation improvements, or code contributions, your help is greatly appreciated.

Please take a moment to review this document to understand how to contribute effectively.

## Code of Conduct

This project adheres to a [Code of Conduct](https://github.com/AbdelrahmanMostafa-Eng/bioenv/blob/main/CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to AbdelrahmanMostafa-Eng@users.noreply.github.com.

## How Can I Contribute?

### Reporting Bugs

If you find a bug, please open an issue on our [GitHub Issues page](https://github.com/AbdelrahmanMostafa-Eng/bioenv/issues) and include:

- A clear and concise description of the bug.
- Steps to reproduce the behavior.
- Expected behavior.
- Actual behavior.
- Your operating system and `bioenv` version.

### Suggesting Enhancements

We love new ideas! If you have a suggestion for an enhancement or a new feature, please open an issue on our [GitHub Issues page](https://github.com/AbdelrahmanMostafa-Eng/bioenv/issues) and include:

- A clear and concise description of the proposed feature.
- Why this feature would be useful.
- Any alternative solutions you've considered.

### Code Contributions

1.  **Fork the Repository:** Start by forking the `bioenv` repository to your GitHub account.
2.  **Clone Your Fork:** Clone your forked repository to your local machine:
    ```bash
    git clone https://github.com/AbdelrahmanMostafa-Eng/bioenv.git
    cd bioenv
    ```
3.  **Create a New Branch:** Create a new branch for your feature or bug fix:
    ```bash
    git checkout -b feature/your-feature-name
    # or
    git checkout -b bugfix/issue-description
    ```
4.  **Make Your Changes:** Implement your changes, ensuring they adhere to the project's coding style and conventions.
    - Run `cargo fmt` to format your code.
    - Run `cargo clippy -- -D warnings` to catch common mistakes.
    - Write tests for your changes to ensure correctness and prevent regressions.
5.  **Run Tests:** Before committing, ensure all tests pass:
    ```bash
    cargo test
    ```
6.  **Commit Your Changes:** Write clear, concise commit messages. We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification (e.g., `feat: add new feature`, `fix: resolve bug`).
    ```bash
    git commit -m "feat: add new feature"
    ```
7.  **Push to Your Fork:**
    ```bash
    git push origin feature/your-feature-name
    ```
8.  **Create a Pull Request:** Open a Pull Request from your forked repository to the `main` branch of the original `bioenv` repository. Provide a clear title and description of your changes.

## Style Guides

-   **Rust:** We follow the official [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md) enforced by `rustfmt`.
-   **Clippy:** All Clippy warnings (`-D warnings`) must be resolved.

## Code Review Process

All pull requests will be reviewed by the maintainers. We may ask for changes or clarifications. Once approved, your changes will be merged.

## Security Vulnerabilities

If you discover a security vulnerability, please report it responsibly. See `SECURITY.md` for details.

Thank you for contributing to `BioEnv`!
