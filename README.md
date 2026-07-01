# supcat
[![Release](https://github.com/raiyanu/supcat/actions/workflows/release.yml/badge.svg)](https://github.com/raiyanu/supcat/actions/workflows/release.yml)

`supcat` is a modern, cross-platform terminal-based AI context preparer. It allows you to recursively explore directories, interactively select files or folders, preview contents with syntax highlighting, and export selected files into a single structured, AI-ready context format. The output can be written to standard output or copied directly to your clipboard.

## Features

- **Split-Pane Layout**: An interactive terminal UI (TUI) featuring a left-pane directory tree explorer and a right-pane code preview window.
- **Multi-Selection Checkboxes**: Select or deselect files and entire subtrees recursively (`☑` / `☐`).
- **File & Folder Filtering**: Instantly search and filter the directory tree using `/`.
- **Previews & Syntax Highlighting**: Preview code files under 1MB with a lightweight built-in syntax highlighter for keywords, strings, and comments (skips binary files safely).
- **Gitignore Respect**: Leverages gitignore rules, supporting `.gitignore` and `.ignore` (both global and parent) by default.
- **Multiple Output Formats**: Export context in **Plain Text**, **Markdown** (with language-specific code blocks), **XML** (using `<file path="...">` tags), or **JSON** (as escaped key-value mappings).
- **Clipboard Integration**: Automatically copies output to your system clipboard (`pbcopy`/`xclip`/`xsel`/`wl-copy`/`clip` are automatically detected and used).

---

## Installation

### 1. From Official APT Repository (Ubuntu / Debian)

You can install `supcat` and keep it updated via the official APT repository hosted on GitHub Pages:

```bash
# 1. Download and add the GPG signing key
curl -fsSL https://raiyanu.github.io/supcat/public.key \
  | sudo gpg --dearmor -o /usr/share/keyrings/supcat.gpg

# 2. Add the APT repository source
echo "deb [signed-by=/usr/share/keyrings/supcat.gpg] https://raiyanu.github.io/supcat stable main" \
  | sudo tee /etc/apt/sources.list.d/supcat.list

# 3. Update package lists and install supcat
sudo apt update
sudo apt install supcat
```

### 2. Download from GitHub Releases

Pre-compiled binaries, installers, and `.deb` packages are generated for every release. You can download them directly from the [GitHub Releases Page](https://github.com/raiyanu/supcat/releases).

Supported platform targets:
- **Linux**: `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-gnu`
- **macOS**: `x86_64-apple-darwin`, `aarch64-apple-darwin`
- **Windows**: `x86_64-pc-windows-msvc`

### 3. Build from Source

If you have Rust and Cargo installed, you can build `supcat` from source:

```bash
# Clone the repository
git clone https://github.com/raiyanu/supcat.git
cd supcat

# Build the release binary
cargo build --release
```

The compiled binary will be available at `target/release/supcat`. You can move it into your system's `PATH` (e.g., `/usr/local/bin/`).

---

## Usage

Run `supcat` in your terminal, optionally specifying a starting path:

```bash
supcat [options] [path]
```

### Options

* `-h`, `--help` : Show help message and exit.
* `-a`, `--hidden` : Include hidden files and directories in traversal.
* `--no-gitignore` : Do not respect `.gitignore` or `.ignore` rules.
* `--symlinks` : Follow symbolic links during traversal.
* `--max-depth <depth>` : Limit the directory traversal depth.
* `--format <format>` : Set the initial formatting option: `plain`, `markdown`/`md`, `xml`, `json` (default is `plain`).
* `-c`, `--clipboard` : Copy final context to the clipboard instead of printing to stdout.

---

## Interactive TUI Controls

When in the TUI, the following keyboard controls are supported:

| Key | Action |
|---|---|
| `↑` / `↓` | Move cursor up or down |
| `←` / `→` | Collapse or expand directory |
| `Space` | Toggle selection of the current item (runs recursively for directories) |
| `Tab` | Toggle selection of the current subtree |
| `a` | Select or deselect all items in the workspace |
| `/` | Open the search prompt to filter the file explorer |
| `f` | Cycle through output formats (Plain Text, Markdown, XML, JSON) |
| `PageUp` / `PageDown` | Scroll code preview window by pages |
| `[` / `]` | Scroll code preview window by lines |
| `Enter` | Confirm selection, export context to clipboard/stdout, and exit |
| `q` / `Esc` | Quit without exporting |

---

## License

This project is licensed under the [MIT License](LICENSE).