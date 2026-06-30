pub mod state;
pub use state::TuiState;

use common::{Node, OutputFormat};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::path::Path;

pub fn run(root: Node, initial_format: OutputFormat) -> std::io::Result<Option<(Node, OutputFormat)>> {
    enable_raw_mode()?;

    let mut stdout = std::io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut state = TuiState::new(root, initial_format);

    loop {
        let height = terminal.size()?.height.saturating_sub(4) as usize;
        if state.cursor_index >= state.scroll_offset + height {
            state.scroll_offset = state.cursor_index - height + 1;
        } else if state.cursor_index < state.scroll_offset {
            state.scroll_offset = state.cursor_index;
        }

        terminal.draw(|f| {
            let area = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(area);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(chunks[0]);

            let visible_range_start = state.scroll_offset;
            let visible_range_end = std::cmp::min(state.visible_items.len(), state.scroll_offset + height);
            let mut explorer_lines = Vec::new();

            if !state.visible_items.is_empty() {
                let range = &state.visible_items[visible_range_start..visible_range_end];
                for (idx, item) in range.iter().enumerate() {
                    let actual_idx = state.scroll_offset + idx;
                    let is_cursor = actual_idx == state.cursor_index;

                    let checkbox = if item.selected { "☑" } else { "☐" };
                    let expand_icon = if item.is_dir {
                        if item.expanded { "▼ " } else { "▶ " }
                    } else {
                        "  "
                    };

                    let line_text = format!(
                        "{}{}{} {}",
                        "  ".repeat(item.depth),
                        expand_icon,
                        checkbox,
                        item.name
                    );

                    let style = if is_cursor {
                        Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
                    } else if item.is_dir {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    } else if item.selected {
                        Style::default().fg(Color::Green)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    explorer_lines.push(Line::from(Span::styled(line_text, style)));
                }
            }

            let explorer_block = Block::default()
                .title(" supcat - Explorer ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let explorer_para = Paragraph::new(explorer_lines).block(explorer_block);
            f.render_widget(explorer_para, main_chunks[0]);

            let preview_height = main_chunks[1].height.saturating_sub(2) as usize;
            let preview_lines_range = if state.preview_lines.is_empty() {
                &[]
            } else {
                let start = std::cmp::min(state.preview_scroll_offset, state.preview_lines.len().saturating_sub(1));
                let end = std::cmp::min(start + preview_height, state.preview_lines.len());
                &state.preview_lines[start..end]
            };

            let ext = state.preview_path.as_ref()
                .and_then(|p| p.extension())
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();

            let mut preview_text = Vec::new();
            for (idx, line) in preview_lines_range.iter().enumerate() {
                let line_num = state.preview_scroll_offset + idx + 1;
                let num_span = Span::styled(
                    format!("{:>3} │ ", line_num),
                    Style::default().fg(Color::DarkGray)
                );

                let mut line_spans = highlight_line(line, &ext).spans;
                line_spans.insert(0, num_span);
                preview_text.push(Line::from(line_spans));
            }

            let preview_title = format!(
                " supcat - Preview [{}] ",
                state.preview_path.as_ref()
                    .map(|p| p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| p.display().to_string()))
                    .unwrap_or_else(|| "None".to_string())
            );
            let preview_block = Block::default()
                .title(preview_title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let preview_para = Paragraph::new(preview_text).block(preview_block).wrap(Wrap { trim: false });
            f.render_widget(preview_para, main_chunks[1]);

            let status_style = Style::default().bg(Color::Indexed(235)).fg(Color::White);
            let status_line = if state.search_mode {
                let cursor_char = "▊";
                Line::from(vec![
                    Span::styled(" Search: /", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                    Span::styled(state.search_query.clone(), Style::default().fg(Color::White)),
                    Span::styled(cursor_char, Style::default().fg(Color::Yellow)),
                ])
            } else {
                let selected_count = count_selected(&state.root);
                let format_str = match state.format {
                    OutputFormat::Plain => "Plain",
                    OutputFormat::Markdown => "Markdown",
                    OutputFormat::Xml => "XML",
                    OutputFormat::Json => "JSON",
                };
                Line::from(vec![
                    Span::styled(format!(" Format: {} ", format_str), Style::default().bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" | Selected: {} items ", selected_count), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(" | ↑↓ Move | ← Collapse | → Expand | Space Select | Tab Subtree | a Toggle All | / Search | f Format | Enter Export | q Quit", Style::default().fg(Color::Gray)),
                ])
            };
            let status_para = Paragraph::new(status_line).style(status_style);
            f.render_widget(status_para, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if state.search_mode {
                match key.code {
                    KeyCode::Enter => {
                        state.search_mode = false;
                        state.update_visible();
                    }
                    KeyCode::Esc => {
                        state.search_mode = false;
                        state.search_query.clear();
                        state.update_visible();
                    }
                    KeyCode::Backspace => {
                        state.search_query.pop();
                        state.update_visible();
                    }
                    KeyCode::Char(c) => {
                        state.search_query.push(c);
                        state.update_visible();
                    }
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        break;
                    }
                    KeyCode::Char('f') => {
                        state.format = match state.format {
                            OutputFormat::Plain => OutputFormat::Markdown,
                            OutputFormat::Markdown => OutputFormat::Xml,
                            OutputFormat::Xml => OutputFormat::Json,
                            OutputFormat::Json => OutputFormat::Plain,
                        };
                    }
                    KeyCode::Char('a') => {
                        let all_selected = is_all_selected(&state.root);
                        set_all_selected(&mut state.root, !all_selected);
                        state.update_visible();
                    }
                    KeyCode::Up => {
                        if state.cursor_index > 0 {
                            state.cursor_index -= 1;
                            state.update_preview();
                        }
                    }
                    KeyCode::Down => {
                        if !state.visible_items.is_empty() && state.cursor_index < state.visible_items.len() - 1 {
                            state.cursor_index += 1;
                            state.update_preview();
                        }
                    }
                    KeyCode::Left => {
                        if !state.visible_items.is_empty() {
                            let path = &state.visible_items[state.cursor_index].path;
                            if let Some(node) = find_node_mut(&mut state.root, path) {
                                if node.is_dir && node.expanded {
                                    node.expanded = false;
                                } else {
                                    if let Some(parent_path) = path.parent() {
                                        if let Some(parent_idx) = state.visible_items.iter().position(|item| &item.path == parent_path) {
                                            state.cursor_index = parent_idx;
                                        }
                                    }
                                }
                            }
                            state.update_visible();
                        }
                    }
                    KeyCode::Right => {
                        if !state.visible_items.is_empty() {
                            let path = &state.visible_items[state.cursor_index].path;
                            if let Some(node) = find_node_mut(&mut state.root, path) {
                                if node.is_dir && !node.expanded {
                                    node.expanded = true;
                                }
                            }
                            state.update_visible();
                        }
                    }
                    KeyCode::Char(' ') => {
                        if !state.visible_items.is_empty() {
                            let path = &state.visible_items[state.cursor_index].path;
                            if let Some(node) = find_node_mut(&mut state.root, path) {
                                let new_val = !node.selected;
                                if node.is_dir {
                                    set_selected_recursive(node, new_val);
                                } else {
                                    node.selected = new_val;
                                }
                            }
                            state.update_visible();
                        }
                    }
                    KeyCode::Tab => {
                        if !state.visible_items.is_empty() {
                            let path = &state.visible_items[state.cursor_index].path;
                            if let Some(node) = find_node_mut(&mut state.root, path) {
                                if node.is_dir {
                                    let new_val = !node.selected;
                                    set_selected_recursive(node, new_val);
                                }
                            }
                            state.update_visible();
                        }
                    }
                    KeyCode::Char('/') => {
                        state.search_mode = true;
                        state.search_query.clear();
                    }
                    KeyCode::PageDown => {
                        let height = terminal.size()?.height.saturating_sub(4) as usize;
                        state.preview_scroll_offset = std::cmp::min(
                            state.preview_scroll_offset + height,
                            state.preview_lines.len().saturating_sub(1)
                        );
                    }
                    KeyCode::PageUp => {
                        let height = terminal.size()?.height.saturating_sub(4) as usize;
                        state.preview_scroll_offset = state.preview_scroll_offset.saturating_sub(height);
                    }
                    KeyCode::Char(']') => {
                        state.preview_scroll_offset = std::cmp::min(
                            state.preview_scroll_offset + 1,
                            state.preview_lines.len().saturating_sub(1)
                        );
                    }
                    KeyCode::Char('[') => {
                        state.preview_scroll_offset = state.preview_scroll_offset.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        state.exit_with_output = true;
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if state.exit_with_output {
        Ok(Some((state.root, state.format)))
    } else {
        Ok(None)
    }
}

fn count_selected(node: &Node) -> usize {
    let mut count = if !node.is_dir && node.selected { 1 } else { 0 };
    for child in &node.children {
        count += count_selected(child);
    }
    count
}

fn is_all_selected(node: &Node) -> bool {
    if !node.is_dir && !node.selected {
        return false;
    }
    for child in &node.children {
        if !is_all_selected(child) {
            return false;
        }
    }
    true
}

fn set_all_selected(node: &mut Node, selected: bool) {
    node.selected = selected;
    for child in &mut node.children {
        set_all_selected(child, selected);
    }
}

fn set_selected_recursive(node: &mut Node, selected: bool) {
    node.selected = selected;
    for child in &mut node.children {
        set_selected_recursive(child, selected);
    }
}

fn find_node_mut<'a>(node: &'a mut Node, path: &Path) -> Option<&'a mut Node> {
    if node.path == path {
        return Some(node);
    }
    for child in &mut node.children {
        if let Some(found) = find_node_mut(child, path) {
            return Some(found);
        }
    }
    None
}

fn highlight_line(line: &str, ext: &str) -> Line<'static> {
    let mut spans = Vec::new();
    let keywords = match ext {
        "rs" => vec!["fn", "let", "mut", "pub", "struct", "impl", "use", "mod", "match", "if", "else", "return", "true", "false", "for", "in", "loop", "while", "as", "const", "static", "type", "enum", "crate", "super", "self", "Self"],
        "js" | "ts" | "jsx" | "tsx" => vec!["function", "let", "const", "var", "import", "export", "default", "class", "extends", "constructor", "return", "if", "else", "for", "while", "do", "switch", "case", "break", "continue", "new", "this", "true", "false", "null", "undefined", "typeof", "instanceof", "async", "await"],
        "py" => vec!["def", "class", "import", "from", "as", "return", "if", "elif", "else", "for", "in", "while", "try", "except", "finally", "with", "lambda", "global", "nonlocal", "true", "false", "none", "and", "or", "not", "is", "pass", "break", "continue"],
        "go" => vec!["func", "package", "import", "type", "struct", "interface", "var", "const", "return", "if", "else", "for", "range", "switch", "case", "default", "select", "chan", "go", "defer", "map", "nil", "true", "false"],
        _ => vec![],
    };

    let chars: Vec<char> = line.chars().collect();
    let mut comment_start = None;
    if ext == "py" || ext == "sh" || ext == "yml" || ext == "yaml" || ext == "toml" {
        if let Some(pos) = line.find('#') {
            comment_start = Some(pos);
        }
    } else if ext == "html" || ext == "xml" {
        if let Some(pos) = line.find("<!--") {
            comment_start = Some(pos);
        }
    } else {
        if let Some(pos) = line.find("//") {
            comment_start = Some(pos);
        }
    }

    if let Some(pos) = comment_start {
        let before: String = chars[..pos].iter().collect();
        let comment: String = chars[pos..].iter().collect();
        spans.extend(highlight_code_spans(&before, &keywords));
        spans.push(Span::styled(comment, Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC)));
    } else {
        spans.extend(highlight_code_spans(line, &keywords));
    }

    Line::from(spans)
}

fn highlight_code_spans(code: &str, keywords: &[&str]) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current_token = String::new();
    let mut in_string = false;
    let mut string_char = '"';

    let chars: Vec<char> = code.chars().collect();
    let mut idx = 0;

    while idx < chars.len() {
        let c = chars[idx];
        if in_string {
            current_token.push(c);
            if c == string_char && (idx == 0 || chars[idx-1] != '\\') {
                in_string = false;
                spans.push(Span::styled(current_token.clone(), Style::default().fg(Color::Yellow)));
                current_token.clear();
            }
            idx += 1;
        } else {
            if c == '"' || c == '\'' {
                if !current_token.is_empty() {
                    spans.push(flush_token(&current_token, keywords));
                    current_token.clear();
                }
                in_string = true;
                string_char = c;
                current_token.push(c);
                idx += 1;
            } else if c.is_alphanumeric() || c == '_' {
                current_token.push(c);
                idx += 1;
            } else {
                if !current_token.is_empty() {
                    spans.push(flush_token(&current_token, keywords));
                    current_token.clear();
                }
                spans.push(Span::styled(c.to_string(), Style::default().fg(Color::White)));
                idx += 1;
            }
        }
    }

    if !current_token.is_empty() {
        if in_string {
            spans.push(Span::styled(current_token, Style::default().fg(Color::Yellow)));
        } else {
            spans.push(flush_token(&current_token, keywords));
        }
    }

    spans
}

fn flush_token(token: &str, keywords: &[&str]) -> Span<'static> {
    if keywords.contains(&token) {
        Span::styled(token.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    } else if token.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
        Span::styled(token.to_string(), Style::default().fg(Color::Magenta))
    } else {
        Span::styled(token.to_string(), Style::default().fg(Color::White))
    }
}