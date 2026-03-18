# Contributing to SafeSurf Protocol 🌊

First off, thank you for considering contributing to the SafeSurf Protocol! It's people like you who make the open-source community such an amazing place to learn, inspire, and create.

The SafeSurf Protocol is a critical security-first project. Every contribution helps make the "Invisible Web" safer for everyone.

---

## 🧭 Vision

SafeSurf Protocol (SSP) aims to provide a high-fidelity, content-aware safety layer for anonymity networks. We focus on:
- **Zero-Trust Content Handling**: Sanitizing everything by default.
- **Privacy Preservation**: Minimizing metadata leakage.
- **Performance**: Ensuring that safety doesn't come at the cost of usability.

---

## 📜 Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md) (standard Contributor Covenant). Please be respectful and professional in all interactions.

---

## 🛠 Getting Started

### Prerequisites
- **Rust Toolchain**: You'll need the latest stable version of Rust. Install it via [rustup](https://rustup.rs/).
- **Cargo-Audit**: For scanning dependencies for vulnerabilities.
  ```bash
  cargo install cargo-audit
  ```
- **Tor Daemon**: If you plan on testing real-world proxying.

### Local Development Setup
1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/the-shadow-0/SafeSurf-protocol.git
   cd SafeSurf-protocol
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test --all
   ```

---

## 🤝 How to Contribute

### Reporting Bugs
- Check the [Issues](https://github.com/the-shadow-0/SafeSurf-protocol/issues) to see if the bug has already been reported.
- If not, create a new issue using the "Bug Report" template.
- Include as much detail as possible: steps to reproduce, OS version, and any relevant logs.

### Suggesting Enhancements
- Open an [Issue](https://github.com/the-shadow-0/SafeSurf-protocol/issues) with the tag `enhancement`.
- Describe the feature, why it's needed, and how it fits into the SSP vision.

### Pull Requests
1. Create a new branch for your work: `git checkout -b feature/amazing-feature`.
2. Ensure your code follows our **Coding Standards** (see below).
3. Add tests for any new functionality.
4. Update documentation if necessary.
5. Push to your fork and submit a Pull Request (PR).

---

## 🏗 Coding Standards

### Security Mindset
Safety is in our name. When writing code, always consider:
- **Panic avoidance**: Use `Result` and `Option` instead of `unwrap()`.
- **Memory Safety**: Avoid `unsafe` blocks unless absolutely necessary and documented.
- **Input Validation**: Never trust external data (HTML, network frames).

### Linting & Formatting
We use standard Rust tools to maintain code quality:
- **Clippy**: Run `cargo clippy --all-targets --all-features -- -D warnings`. Your PR must pass clippy.
- **Rustfmt**: Run `cargo fmt --all`. Use the default configurations.

### Commit Messages
- Use the imperative mood ("Add feature" instead of "Added feature").
- Keep the first line under 50 characters.
- Reference issues if applicable (e.g., `Fixes #42`).

---

## 🧪 Testing

Every PR must be covered by tests.
- **Unit Tests**: For core logic in `safe-surf-core`.
- **Integration Tests**: For daemon/CLI interaction.
- **Smoke Tests**: We use `demo.sh` to verify the end-to-end flow.

---

## 💬 Communication

If you have questions or want to discuss a major change before diving in, please reach out via GitHub Discussions or open a placeholder issue.

---

*Keep surfing safely!*
