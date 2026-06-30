use common::{Node, OutputFormat};

pub fn format_context(root: &Node, format: OutputFormat) -> String {
    let mut selected_files = Vec::new();
    collect_selected_files(root, &mut selected_files);

    if selected_files.is_empty() {
        return String::new();
    }

    match format {
        OutputFormat::Plain => {
            let mut out = String::new();
            for file in selected_files {
                let rel_path = file.path.strip_prefix(&root.path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| file.name.clone());
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    out.push_str(&format!("===== {} =====\n", rel_path));
                    out.push_str(&content);
                    if !content.ends_with('\n') {
                        out.push('\n');
                    }
                }
            }
            out
        }
        OutputFormat::Markdown => {
            let mut out = String::new();
            for file in selected_files {
                let rel_path = file.path.strip_prefix(&root.path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| file.name.clone());
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    let ext = file.path.extension()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default();
                    out.push_str(&format!("## {}\n\n", rel_path));
                    out.push_str(&format!("```{}\n", ext));
                    out.push_str(&content);
                    if !content.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push_str("```\n\n");
                }
            }
            out
        }
        OutputFormat::Xml => {
            let mut out = String::new();
            for file in selected_files {
                let rel_path = file.path.strip_prefix(&root.path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| file.name.clone());
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    out.push_str(&format!("<file path=\"{}\">\n", rel_path));
                    out.push_str(&content);
                    if !content.ends_with('\n') {
                        out.push('\n');
                    }
                    out.push_str("</file>\n");
                }
            }
            out
        }
        OutputFormat::Json => {
            let mut out = String::new();
            out.push_str("{\n");
            let mut first = true;
            for file in selected_files {
                let rel_path = file.path.strip_prefix(&root.path)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| file.name.clone());
                if let Ok(content) = std::fs::read_to_string(&file.path) {
                    if !first {
                        out.push_str(",\n");
                    }
                    first = false;
                    out.push_str(&format!("  \"{}\": \"{}\"", escape_json(&rel_path), escape_json(&content)));
                }
            }
            out.push_str("\n}");
            out
        }
    }
}

fn collect_selected_files<'a>(node: &'a Node, out: &mut Vec<&'a Node>) {
    if !node.is_dir && node.selected {
        out.push(node);
    }
    for child in &node.children {
        collect_selected_files(child, out);
    }
}

fn escape_json(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}