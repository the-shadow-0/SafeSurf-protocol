# SafeSurf Protocol (Reference Implementation)

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/the-shadow-0/SafeSurf-protocol)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](https://github.com/the-shadow-0/SafeSurf-protocol/blob/main/LICENSE)
[![Safety](https://img.shields.io/badge/safety-checked-orange)](https://github.com/the-shadow-0/SafeSurf-protocol/blob/main/SECURITY.md)

**SafeSurf Protocol** is a defensive, privacy-first safety layer designed to protect users navigating risky networks (including the dark net/deep web). It mitigates metadata leakage, neutralizes malicious content, protects credentials via a secure vault, and provides explainable risk scoring.

---

## 🚀 Step-by-Step Quickstart

Follow these steps to get the reference implementation running in production mode.

### 1. Prerequisites
- **Rust**: Install via [rustup.rs](https://rustup.rs/)
- **Cargo Audit** (Optional): `cargo install cargo-audit`

### 2. Installation
Clone the repository and build the workspace:
```bash
git clone https://github.com/the-shadow-0/SafeSurf-protocol.git
cd SafeSurf-protocol
cargo build --release
```

### 3. Production Deployment (Linux)
SafeSurf can be managed as a system service:
```bash
# Register and start the service
sudo cp safe-surfd.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable --now safe-surfd

# Check status
sudo systemctl status safe-surfd
```

### 4. Global Network Protection
To route all system traffic (Browsers, CLI, Tor) through the SafeSurf pipeline:
```bash
# Auto-configure system proxy (GNOME/Manual)
./target/release/safe_surf_cli sys-setup --enable

# To disable
./target/release/safe_surf_cli sys-setup --enable false
```
*Note: This starts the HTTP Proxy on `127.0.0.1:8080`. For CLI tools like `curl`, ensure `http_proxy` is exported.*

### 5. Interact via CLI (`safe_surf_cli`)
Open a new terminal and use the CLI to interact with the daemon.

- **Check Status**:
  ```bash
  ./target/release/safe_surf_cli status
  ```
- **Initialize a Secure Session**:
  ```bash
  ./target/release/safe_surf_cli init
  ```
- **Analyze Page Risk**:
  ```bash
  ./target/release/safe_surf_cli risk --url "http://example.onion" --file path/to/sample.html
  ```
- **Sanitize Content**:
  ```bash
  ./target/release/safe_surf_cli sanitize --url "http://example.onion" --content "<html><script>alert('malicious')</script><body>Safe Content</body></html>"
  ```

---

## 🛠 In-Depth Integration Guide

### 1. Tor Browser Integration
SafeSurf follows a **Defense-in-Depth** model: **Browser -> SafeSurf -> Tor**.

#### **Proxy Chaining (Recommended)**:
1.  Open Tor Browser Settings.
2.  Navigate to **Network Settings** -> **Settings...** (Configure how Tor Browser connects).
3.  Configure a **Manual Proxy Configuration**:
    - **HTTP Proxy**: `127.0.0.1`, **Port**: `3000` (Point to `safe_surfd`).
    - Use this proxy server for all protocols.
4.  Ensure `safe_surfd` is configured to fetch via SOCKS5 `127.0.0.1:9050` (Local Tor).
5.  **Result**: Every page you visit is sanitized and risk-scored *before* the browser renders it.

#### **Browser Extension**:
Load the [Extension Stub](file:///home/ai-creator/Bureau/Github-Projects/SafeSurf%20Protocol/examples/browser-extension-stub) into the browser to get real-time safety scores in the UI.

### 2. Headless & CLI Tools (Advanced Users)
If you don't use a browser, SafeSurf protects you from malicious payloads served to your scripts.

#### **Pattern: Safety Microservice**
Your script (Python/Go/Bash) fetches raw data from Tor and then "washes" it through SafeSurf:
```bash
RAW_HTML=$(curl --socks5-hostname 127.0.0.1:9050 http://example.onion)
SAFE_HTML=$(curl -X POST -H "Content-Type: application/json" \
  -d "{\"url\":\"http://example.onion\", \"html\":\"$RAW_HTML\"}" \
  http://127.0.0.1:3000/content/sanitize)
```

#### **Pattern: Embedded Rust SDK**
Import `safe_surf_core` into your Rust project to use the logic locally without the daemon:
```rust
use safe_surf_core::sanitization::ContentSanitizer;
let safe_content = ContentSanitizer::default().sanitize(raw_input);
```

---

## 🔒 Security & Privacy Features

-   **Crypto Customization**: Configure Argon2id (m_cost, t_cost) and XChaCha20 implementation via `safe_surf_core/src/config.rs`.
-   **Credential Vault**: Local-first storage using AEAD encryption and blinded indicator matching.
-   **Traffic Hardening**: Configurable timing jitter and cover traffic stubs to fight metadata analysis.
-   **Session Isolation**: Per-tab ephemeral keys that are zeroized in memory upon termination.

---

## ⚖️ Ethical Appendix & Safety Rules

- **Defensive Only**: This tool is designed to protect users. It does **not** provide instructions for accessing illegal services or bypassing lawful restrictions.
- **Synthentic Data**: All demos and examples use synthetic data.
- **Responsible Disclosure**: If you find a security flaw, please see our [SECURITY.md](file:///home/ai-creator/Bureau/Github-Projects/SafeSurf%20Protocol/SECURITY.md).

---

## 📄 License
This project is dual-licensed under the [MIT License](LICENSE-MIT) and the [Apache License 2.0](LICENSE-APACHE).
