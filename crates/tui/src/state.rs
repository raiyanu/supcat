use common::{Node, VisibleNode, OutputFormat};
use std::path::PathBuf;

pub struct TuiState {
    pub root: Node,
    pub visible_items: Vec<VisibleNode>,
    pub cursor_index: usize,
    pub scroll_offset: usize,
    pub preview_scroll_offset: usize,
    pub search_query: String,
    pub search_mode: bool,
    pub format: OutputFormat,
    pub preview_lines: Vec<String>,
    pub preview_path: Option<PathBuf>,
    pub exit_with_output: bool,
}

impl TuiState {
    pub fn new(root: Node, format: OutputFormat) -> Self {
        let visible_items = core::flatten(&root);
        let mut state = Self {
            root,
            visible_items,
            cursor_index: 0,
            scroll_offset: 0,
            preview_scroll_offset: 0,
            search_query: String::new(),
            search_mode: false,
            format,
            preview_lines: Vec::new(),
            preview_path: None,
            exit_with_output: false,
        };
        state.update_preview();
        state
    }

    pub fn update_visible(&mut self) {
        if self.search_mode && !self.search_query.is_empty() {
            self.visible_items = core::flatten_filtered(&self.root, &self.search_query);
        } else {
            self.visible_items = core::flatten(&self.root);
        }

        if self.visible_items.is_empty() {
            self.cursor_index = 0;
        } else if self.cursor_index >= self.visible_items.len() {
            self.cursor_index = self.visible_items.len() - 1;
        }

        self.update_preview();
    }

    pub fn update_preview(&mut self) {
        if self.visible_items.is_empty() {
            self.preview_lines = Vec::new();
            self.preview_path = None;
            self.preview_scroll_offset = 0;
            return;
        }

        let path = &self.visible_items[self.cursor_index].path;
        if self.preview_path.as_ref() == Some(path) {
            return; // Already loaded
        }

        self.preview_path = Some(path.clone());
        self.preview_scroll_offset = 0;

        self.preview_lines = if path.is_dir() {
            let mut lines = Vec::new();
            lines.push(format!("Folder: {}", path.display()));
            if let Ok(entries) = std::fs::read_dir(path) {
                let mut count = 0;
                let mut size = 0;
                for entry in entries {
                    if let Ok(entry) = entry {
                        count += 1;
                        if let Ok(metadata) = entry.metadata() {
                            if metadata.is_file() {
                                size += metadata.len();
                            }
                        }
                    }
                }
                lines.push(format!("Immediate children: {}", count));
                lines.push(format!("Total size of files: {} bytes", size));
            }
            lines
        } else {
            let metadata = match std::fs::metadata(path) {
                Ok(m) => m,
                Err(e) => {
                    self.preview_lines = vec![format!("Error reading metadata: {}", e)];
                    return;
                }
            };

            let size = metadata.len();
            if size > 1_000_000 {
                vec![
                    format!("File: {}", path.display()),
                    format!("Size: {} bytes", size),
                    "WARNING: File is too large to preview (>1MB)".to_string(),
                ]
            } else {
                match std::fs::read(path) {
                    Ok(bytes) => {
                        let check_len = std::cmp::min(bytes.len(), 1024);
                        if bytes[..check_len].contains(&0) {
                            vec![
                                format!("File: {}", path.display()),
                                format!("Size: {} bytes", size),
                                "WARNING: Binary file detected. Preview disabled.".to_string(),
                            ]
                        } else {
                            match String::from_utf8(bytes) {
                                Ok(content) => content.lines().map(|s| s.to_string()).collect(),
                                Err(_) => vec![
                                    format!("File: {}", path.display()),
                                    "WARNING: File contains invalid UTF-8 characters.".to_string(),
                                ],
                            }
                        }
                    }
                    Err(e) => vec![format!("Error reading file: {}", e)],
                }
            }
        };
    }
}