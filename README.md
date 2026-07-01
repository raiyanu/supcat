<div align="center">

# 🐱 supcat

**A blazing-fast, cross-platform terminal UI for preparing AI-ready context from your codebase.**

Recursively explore directories, interactively select files, preview with syntax highlighting, and export a clean, structured context — ready to paste into any LLM.

[![Release](https://github.com/raiyanu/supcat/actions/workflows/release.yml/badge.svg)](https://github.com/raiyanu/supcat/actions/workflows/release.yml)
[![Rust Unit Test](https://github.com/raiyanu/supcat/actions/workflows/rust.yml/badge.svg)](https://github.com/raiyanu/supcat/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)

</div>

---

## ✨ Why supcat?

Copy-pasting files into ChatGPT or Claude one by one is slow and error-prone. `supcat` gives you a fast, keyboard-driven TUI to **pick exactly the files you need**, see what you're selecting before you send it, and export it all in one clean, structured block — formatted the way LLMs like it best.

## 🚀 Features

| | |
|---|---|
| 🗂️ **Split-Pane Layout** | Interactive TUI with a directory tree on the left and a live code preview on the right |
| ☑️ **Multi-Selection** | Select or deselect files and entire subtrees recursively (`☑` / `☐`) |
| 🔍 **Instant Filtering** | Search and filter the directory tree on the fly with `/` |
| 🎨 **Syntax Highlighting** | Built-in, lightweight highlighting for keywords, strings, and comments — skips binaries safely, previews files under 1MB |
| 🙈 **Gitignore Aware** | Respects `.gitignore` and `.ignore` rules (global and parent) out of the box |
| 📄 **Multiple Export Formats** | Plain Text, Markdown (with language-tagged code blocks), XML (`<file path="...">`), or JSON |
| 📋 **Clipboard Integration** | Auto-detects and uses `pbcopy` / `xclip` / `xsel` / `wl-copy` / `clip` |

---

## 📦 Installation

### Option 1 — APT Repository (Ubuntu / Debian)

Install and keep `supcat` updated via the official APT repo:

```bash
# 1. Add the GPG signing key
curl -fsSL https://raiyanu.github.io/supcat/public.key \
  | sudo gpg --dearmor -o /usr/share/keyrings/supcat.gpg

# 2. Add the APT source
echo "deb [signed-by=/usr/share/keyrings/supcat.gpg] https://raiyanu.github.io/supcat stable main" \
  | sudo tee /etc/apt/sources.list.d/supcat.list

# 3. Install
sudo apt update
sudo apt install supcat
```

### Option 2 — GitHub Releases

Pre-compiled binaries, installers, and `.deb` packages are published with every release.

👉 **[Download from the Releases page](https://github.com/raiyanu/supcat/releases)**

| Platform | Targets |
|---|---|
| 🐧 Linux | `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-gnu` |
| 🍎 macOS | `x86_64-apple-darwin`, `aarch64-apple-darwin` |
| 🪟 Windows | `x86_64-pc-windows-msvc` |

### Option 3 — Build from Source

```bash
git clone https://github.com/raiyanu/supcat.git
cd supcat
cargo build --release
```

The compiled binary lands at `target/release/supcat` — move it into your `PATH` (e.g. `/usr/local/bin/`) to use it anywhere.

---

## 🛠️ Usage

```bash
supcat [options] [path]
```

| Flag | Description |
|---|---|
| `-h`, `--help` | Show help message and exit |
| `-a`, `--hidden` | Include hidden files and directories in traversal |
| `--no-gitignore` | Ignore `.gitignore` / `.ignore` rules |
| `--symlinks` | Follow symbolic links during traversal |
| `--max-depth <depth>` | Limit directory traversal depth |
| `--format <format>` | Set initial output format: `plain`, `markdown`/`md`, `xml`, `json` (default: `plain`) |
| `-c`, `--clipboard` | Copy the final context to the clipboard instead of printing to stdout |

---

## ⌨️ Interactive TUI Controls

| Key | Action |
|:---:|---|
| `↑` / `↓` | Move cursor up or down |
| `←` / `→` | Collapse or expand directory |
| `Space` | Toggle selection of current item (recursive for directories) |
| `Tab` | Toggle selection of the current subtree |
| `a` | Select or deselect all items |
| `/` | Open search prompt to filter the tree |
| `f` | Cycle through output formats |
| `PageUp` / `PageDown` | Scroll preview by pages |
| `[` / `]` | Scroll preview by lines |
| `Enter` | Export selection to clipboard/stdout and exit |
| `q` / `Esc` | Quit without exporting |

---

## 📄 License

Licensed under the [MIT License](LICENSE).

<div align="center">

Made with 🦀 by [raiyanu](https://github.com/raiyanu)

</div>
