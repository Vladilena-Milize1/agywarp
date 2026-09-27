<div align="center">
    <h1>agywarp</h1>
</div>

<div align="center">

[![License](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80+-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux-lightgrey?style=flat-square)](https://github.com/Vladilena-Milize1/agywarp)
[![Release](https://img.shields.io/badge/Release-v0.2.0--rust-informational?style=flat-square)](https://github.com/Vladilena-Milize1/agywarp)

</div>

<div align="center">
    <a href="readme.md">English</a> | <a href="README.zh-CN.md">简体中文</a>
</div>

## Intro

> [!CAUTION]
> **Disclaimer**: This project does not guarantee stable Antigravity connectivity or independence from specific proxy nodes. Results depend on the selected node, the WARP connection, and the target service. Consider using this only when raw proxy isn't working.

### What

**agywarp** is a cross-platform TUI and CLI tool written in **Rust** that routes selected processes through 
[**Cloudflare WARP**](https://developers.cloudflare.com/warp-client/)'s local proxy and 
[**Clash Verge Rev**](https://github.com/clash-verge-rev/clash-verge-rev)'s [**Mihomo**](https://github.com/MetaCubeX/mihomo) core. It dynamically loads temporary 
routing rules into Mihomo memory and restores the generated base config when
the tunnel is stopped.

### Why

This tool is originally built to proxy [**Google Antigravity**](https://antigravity.google/) CLI and its **VS Code** Extension through Cloudflare WARP, as common proxy services' IP can be easily blocked by Google for being an IDC IP.  

Process-specific WARP routing requires Mihomo rules and the WARP daemon to
work together. agywarp provides one unified control to load the selected process
rules and remove the runtime routing when stopped.

- **Process routing**: Route enabled process groups (Antigravity, Chrome, Gemini, or custom executables) through WARP using Mihomo.
- **Reversible setup**: Remove runtime rules and restore the clean base config on stop without altering disk subscriptions.
- **Cross-platform**: Native support for both **Windows** (via TCP controller & Windows process scanning) and **Linux**.

---

## Getting Started

### Requirements

- A machine running **Windows 10/11** or a **Linux** distribution (x86_64).
- **Clash Verge Rev** with Mihomo running, [**TUN**](https://wiki.metacubex.one/en/config/inbound/tun/) enabled, and process matching enabled.
- [**`warp-cli`**](https://developers.cloudflare.com/warp-client/) installed, running in `WarpProxy` mode on loopback port `40000`:
  ```bash
  warp-cli mode proxy
  warp-cli proxy port 40000
  ```

### Build & Installation

```bash
# Build optimized release binary
cargo build --release

# The compiled executable is located at target/release/agywarp.exe (Windows) or target/release/agywarp (Linux)

# Optionally install globally to system PATH:
cargo install --path .
```

---

## Usage

### 1. TUI Dashboard

Simply run `agywarp` to open the terminal dashboard:

```bash
agywarp
```

* **Keyboard Controls**:
  * `Tab` / `Shift+Tab`: Cycle focus between **Network Card**, **Process Groups**, and **Output Console**.
  * `Space`:
    * When Network Card is focused: **Toggle routing service ON / OFF**.
    * When Process Groups is focused: **Enable / disable selected process group**.
  * `p`: Toggle proxy protocol between `SOCKS5` (default) and `HTTP CONNECT` (when service is OFF).
  * `r`: Refresh node and exit IP verification.
  * `↑` / `↓`: Navigate process list or scroll output console.
  * `q` / `Ctrl+C`: Safely quit (prompts to stop service first if active).

### 2. CLI Mode

| Command | Description |
| :--- | :--- |
| `agywarp status` | Check status of Clash Verge, Mihomo, WARP, and active routing rules |
| `agywarp on` | Headless command to start routing enabled process groups |
| `agywarp off` | Headless command to stop routing and restore clean Mihomo config |
| `agywarp recover` | Clean up dangling rules and recover base config after a crash |
| `agywarp procs` | Scan and list actively running system processes |
| `agywarp trace` | Verify outbound exit IP and WARP status via `1.1.1.1/cdn-cgi/trace` |

---

## License

This project is licensed under the [MIT License](LICENSE).
