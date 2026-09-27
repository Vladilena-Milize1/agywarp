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

## 项目简介

> [!CAUTION]
> **免责声明**：本项目不保证 Google Antigravity 连接的绝对稳定性，也不保证脱离特定机场节点独立工作。实际效果取决于所选机场节点、Cloudflare WARP 连接质量以及目标服务风控策略。建议仅在原生代理节点被目标服务风控屏蔽时使用。

### 什么是 agywarp

**agywarp** 是一款基于 Rust 开发的跨平台终端 UI (TUI) 与 CLI 工具。它可以将指定进程的流量无缝导向本地 [**Cloudflare WARP**](https://developers.cloudflare.com/warp-client/) 本地代理（端口 40000），并通过 [**Clash Verge Rev**](https://github.com/clash-verge-rev/clash-verge-rev) 的 [**Mihomo**](https://github.com/MetaCubeX/mihomo) 内核出站。

该工具通过 Mihomo Controller API 将动态分流规则与临时本地代理加载到内存中，并在停止隧道时完全恢复原有的订阅基础配置，不改写磁盘订阅源。

### 为什么需要它

开发本工具的初衷是解决使用 [**Google Antigravity**](https://antigravity.google/) CLI、VS Code 扩展及 Gemini 命令行工具时遇到的风控问题：

* 普通数据中心 (IDC) 机场节点的 IP 极易被 Google 识别并拦截；
* Cloudflare WARP 具备家宽/原生网络信誉，但官方客户端通常为全局代理，缺乏进程分流与防环路保护；
* **双层路由设计**：
  * **内层应用流量**：Antigravity 等指定进程 $\rightarrow$ Mihomo TUN 虚拟网卡 $\rightarrow$ 动态匹配至本地 WARP 代理 (`127.0.0.1:40000`)；
  * **外层 WARP 隧道**：WARP 守护进程 (`warp-svc` / `warp-svc.exe`) 本身的隧道流量由防环路 Guard 规则保护，经机场选定节点或直连正常出海；
* **配置无损可逆**：所有规则均通过内存热重载注入，退出或异常恢复时一键重置，绝不污染 Clash 基础配置。

---

## 快速上手

### 环境要求

1. **操作系统**：Windows 10/11 或主流 Linux 发行版（x86_64）。
2. **Clash Verge Rev**：
   - 处于运行状态；
   - 开启 **TUN 模式**（虚拟网卡）；
   - 在设置中开启**进程匹配**。
3. **Cloudflare WARP**：
   - 已安装官方 `warp-cli`（Windows 或 Linux 版）；
   - 处于 `WarpProxy` 模式且代理端口为 `40000`：
     ```bash
     warp-cli mode proxy
     warp-cli proxy port 40000
     ```

### 安装与构建

本项目使用 Rust 编写，无任何额外运行时依赖。

#### 1. 从源码编译（推荐）
```bash
# 编译 Release 高性能版本
cargo build --release

# 编译产物位于: target/release/agywarp.exe (Windows) 或 target/release/agywarp (Linux)
```

#### 2. 全局安装到系统 PATH
```bash
cargo install --path .
```
安装完成后，在终端任意路径直接执行 `agywarp` 即可。

---

## 使用指南

### 1. TUI 交互式仪表盘

直接运行程序即可进入终端仪表盘：
```bash
agywarp
```

* **快捷键说明**：
  * `Tab` / `Shift+Tab`：在 **Network Card（网络卡片）**、**Process Groups（进程组列表）** 和 **Output Console（输出控制台）** 之间切换焦点。
  * `Space`（空格键）：
    * 聚焦在 Network Card 时：**开启 / 关闭路由分流服务**（Service ON / OFF）。
    * 聚焦在 Process Groups 时：**启用 / 停用选中的进程组**。
  * `p`：在服务处于 OFF 时切换本地代理协议（`SOCKS5` 默认 / `HTTP CONNECT`）。
  * `r`：手动刷新当前出站节点与网络出口状态。
  * `↑` / `↓`：在进程组列表中移动光标，或在控制台中滚动查看日志。
  * `q` / `Ctrl+C`：安全退出（若分流正在运行，会保护性提示先按空格关闭）。

### 2. CLI 命令行模式

除交互式 TUI 外，还提供一系列轻量级命令行工具：

| 命令 | 说明 |
| :--- | :--- |
| `agywarp status` | 查看当前 Clash Verge、Mihomo、WARP 和规则的运行状态 |
| `agywarp on` | 无界面一键启动分流规则（支持 `--port` 与 `--mode` 参数） |
| `agywarp off` | 一键关闭分流并复原 Mihomo 基础配置 |
| `agywarp recover` | 异常中断或崩溃后，一键重置 Mihomo 并清理残留规则与会话 |
| `agywarp procs` | 扫描并列出当前系统所有运行中的进程（方便排查实际可执行文件名） |
| `agywarp trace` | 测试本地 WARP 代理出口的 IP、机房位置 (Colo) 与延迟 |

---

## 配置文件路径

| 平台 | 进程组配置 (`profiles.json`) | Clash Verge 基础配置目录 |
| :--- | :--- | :--- |
| **Windows** | `%APPDATA%\agywarp\profiles.json` | `%APPDATA%\io.github.clash-verge-rev.clash-verge-rev` |
| **Linux** | `~/.config/agywarp/profiles.json` | `~/.local/share/io.github.clash-verge-rev.clash-verge-rev` |

---

## 开源协议

本项目基于 [MIT 许可证](LICENSE) 开源。
