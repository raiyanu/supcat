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

Workspace builds successfully.

```
cargo build
cargo run -p supcat
```

working.

---

## 2. Recursive filesystem walker

Implemented recursive directory traversal.

Current:

- directories
- files
- recursive traversal
- alphabetical ordering
- directories before files

Temporary ignore list:

- `.git`
- `target`
- `node_modules`

Future:

Replace temporary ignore list with full `.gitignore` parser.

---

## 3. Tree model

Implemented `Node`.

Current fields:

- name
- path
- is_dir
- expanded
- selected
- children

Tree stored completely in memory.

---

## 4. Flatten layer

Implemented conversion:

```
Tree
↓

Visible list
```

Produces list of visible nodes.

Foundation for:

- scrolling
- cursor
- selection
- rendering

without walking recursive tree every frame.

---

## 5. VisibleNode

Created lightweight render model.

Contains:

- name
- depth
- is_dir
- expanded

UI renders this instead of full tree.

---

## 6. Terminal UI

Integrated:

- ratatui
- crossterm

Current UI:

- alternate screen
- raw terminal mode
- bordered window
- render loop
- quit with `q`

---

## 7. Tree rendering

Current output:

```
▼ .
  ▶ crates
    Cargo.toml
    ...
```

Uses indentation.

Uses folder icons.

Uses flattened tree.

---

## 8. Expand state

Added `expanded` flag.

Current behavior:

Root expanded.

Children collapsed by default.

Flatten only traverses expanded folders.

Architecture ready for interactive expansion.

---

# Current Status

Working:

- workspace
- filesystem scan
- tree model
- flattening
- terminal UI
- rendering
- collapse architecture
- clean separation between data and UI

Not implemented:

- cursor
- keyboard navigation
- folder expand/collapse interaction
- multi-select
- preview pane
- `.gitignore`
- search
- clipboard
- output formatter

---

# Planned Roadmap

## Phase 1

- cursor
- Up/Down navigation
- current row highlight

## Phase 2

- Left/Right expand-collapse
- Enter expand-collapse

## Phase 3

- Space multi-select
- selected state rendering

## Phase 4

- `.gitignore`
- hidden files
- binary detection

## Phase 5

- right-side preview pane
- syntax highlighting

## Phase 6

- output formatter

Formats:

- plain text
- Markdown
- XML
- JSON

## Phase 7

Export selected files as AI-ready context.

Example:

```
===== src/main.rs =====
...

===== Cargo.toml =====
...
```

---

# Long-term Vision

```
supcat
```

↓

Interactive explorer

↓

Select files

↓

Preview contents

↓

Press Enter

↓

Receive AI-ready context

↓

Paste directly into ChatGPT, Claude, Gemini, Codex, etc.

Future versions may share same core with cloud service while keeping CLI fully functional offline.
