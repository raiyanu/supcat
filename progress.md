# Supcat Progress Report

## Goal

Build cross-platform terminal application for preparing AI context.

Long-term features:

- Interactive file explorer
- Multi-select files/folders
- Respect `.gitignore`
- Folder expand/collapse
- Preview pane
- Export selected files as AI-ready context
- Single Rust binary
- No Node.js runtime
- Future cloud sync using shared core

---

# Current Architecture

Workspace:

```text
supcat/
├── Cargo.toml
├── crates/
│   ├── cli
│   ├── common
│   ├── core
│   ├── tui
│   ├── output
│   ├── fs
│   ├── ignore
│   └── tree
```

Current active crates:

- `cli`
- `common`
- `core`
- `tui`

`fs`, `ignore`, `tree`, `output` reserved for future work.

---

# Implemented

## 1. Rust workspace
Workspace builds and compiles successfully in debug and release profiles.

## 2. FS scan & Walker (utilizing `ignore`)
Implemented a powerful recursive directory explorer leveraging `ignore::WalkBuilder` to respect:
- `.gitignore` and `.ignore` files.
- Global and parent gitignore files.
- Hidden files toggle (`show_hidden`).
- Custom symlinks traversal (`follow_symlinks`).
- Max directory depth traversal limit.
- Sorting order (directories before files, ordered alphabetically).

## 3. Tree model
Highly efficient hierarchical tree structure loaded into memory:
- Handles expanded and collapsed toggle states.
- Selection checkboxes (`☑` / `☐`).
- Subtree selection propagation.

## 4. Flatten layer & VisibleNode
Transforms the hierarchical tree structure into a flat list of visible items dynamically. Supports query-based filtering (`flatten_filtered`) for instant file searches.

## 5. Interactive Terminal UI
Built on `ratatui` and `crossterm` raw terminal mode:
- Split-pane layout: Left tree Explorer, Right file Content/Metadata Preview.
- Auto-scrolling left explorer panel.
- Fully scrollable right preview pane.

## 6. Layout controls & key inputs
- `↑` / `↓`: Move cursor.
- `←` / `→`: Collapse or expand directory.
- `Space`: Select current item (recursive toggle for directories).
- `Tab`: Toggle select current subtree.
- `a`: Select/deselect all.
- `/`: Filter tree via search prompt.
- `f`: Toggle formats (Plain, Markdown, XML, JSON).
- `PageUp`/`PageDown`/`[`/`]`: Scroll code preview window.
- `Enter`: Confirm selection, copy/export, and exit.
- `q` / `Esc`: Close utility.

## 7. Interactive Preview & Syntax Highlighter
- Reads file preview efficiently upon cursor changes.
- Safe-locks: skips loading binary files (null-byte checking) or files larger than 1MB.
- Custom lightweight syntax highlighter highlighting comments, string literals, and language keywords.

## 8. Output Formatter & Clipboard Integration
- Formats final selected files into:
  - **Plain Text**: Standard headers.
  - **Markdown**: With appropriate language block tags.
  - **XML**: Inside `<file path="...">` tags.
  - **JSON**: Safely escaped key-value mappings.
- Leverages a custom command-based clipboard integration (`pbcopy`/`xclip`/`xsel`/`wl-copy`/`clip`) to export contexts directly to the system clipboard, falling back gracefully to stdout.

---

# Current Status

All core phases of the roadmap have been fully implemented, tested, and validated!

- **Explorer & navigation**: 100% complete.
- **Tree expand & collapse**: 100% complete.
- **Multi-select & checkboxes**: 100% complete.
- **.gitignore parsing**: 100% complete.
- **Live scrollable preview**: 100% complete.
- **Syntax highlighting**: 100% complete.
- **AI formatters (Plain, MD, XML, JSON)**: 100% complete.
- **Clipboard copying**: 100% complete.
- **Search filtering**: 100% complete.
