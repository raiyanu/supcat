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

#[cfg(test)]
mod tests {
    use super::*;
    use common::{Node, OutputFormat};

    #[test]
    fn test_escape_json() {
        assert_eq!(escape_json("hello \"world\""), "hello \\\"world\\\"");
        assert_eq!(escape_json("line 1\nline 2"), "line 1\\nline 2");
    }

    #[test]
    fn test_format_context() {
        let temp_dir = std::env::temp_dir();
        let temp_file_path = temp_dir.join("supcat_test_file.rs");
        std::fs::write(&temp_file_path, "fn main() {}").unwrap();

        let mut root = Node::new("temp_dir".to_string(), temp_dir.clone(), true);
        let mut file1 = Node::new("supcat_test_file.rs".to_string(), temp_file_path.clone(), false);
        file1.selected = true;
        root.children.push(file1);

        let plain_out = format_context(&root, OutputFormat::Plain);
        assert!(plain_out.contains("===== supcat_test_file.rs ====="));
        assert!(plain_out.contains("fn main() {}"));

        let md_out = format_context(&root, OutputFormat::Markdown);
        assert!(md_out.contains("## supcat_test_file.rs"));
        assert!(md_out.contains("```rs"));

        let xml_out = format_context(&root, OutputFormat::Xml);
        assert!(xml_out.contains("<file path=\"supcat_test_file.rs\">"));

        let json_out = format_context(&root, OutputFormat::Json);
        assert!(json_out.contains("\"supcat_test_file.rs\":"));
        assert!(json_out.contains("fn main() {}"));

        let _ = std::fs::remove_file(temp_file_path);
    }
}