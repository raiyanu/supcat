use common::{Node, Result};
use ignore::WalkBuilder;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct WalkOptions {
    pub show_hidden: bool,
    pub respect_gitignore: bool,
    pub follow_symlinks: bool,
    pub max_depth: Option<usize>,
}

pub fn walk<P: AsRef<Path>>(root: P, options: &WalkOptions) -> Result<Node> {
    let root_path = root.as_ref().canonicalize().unwrap_or_else(|_| root.as_ref().to_path_buf());
    let root_meta = std::fs::metadata(&root_path)?;
    let root_name = root_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| root_path.display().to_string());

    let mut root_node = Node::new(root_name, root_path.clone(), root_meta.is_dir());

    if !root_meta.is_dir() {
        return Ok(root_node);
    }

    let mut builder = WalkBuilder::new(&root_path);
    builder
        .hidden(!options.show_hidden)
        .git_ignore(options.respect_gitignore)
        .ignore(options.respect_gitignore)
        .parents(options.respect_gitignore)
        .follow_links(options.follow_symlinks)
        .max_depth(options.max_depth);

    for entry in builder.build() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        if path == root_path {
            continue;
        }

        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        insert_node(&mut root_node, path, is_dir);
    }

    sort_tree(&mut root_node);

    Ok(root_node)
}

fn insert_node(root: &mut Node, path: &Path, is_dir: bool) {
    let rel_path = match path.strip_prefix(&root.path) {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut current = root;
    let components: Vec<_> = rel_path.components().collect();
    if components.is_empty() {
        return;
    }

    for i in 0..components.len() {
        let comp = components[i].as_os_str().to_string_lossy().to_string();
        if i == components.len() - 1 {
            if !current.children.iter().any(|c| c.name == comp) {
                current.children.push(Node::new(comp, path.to_path_buf(), is_dir));
            }
        } else {
            if let Some(pos) = current.children.iter().position(|c| c.name == comp && c.is_dir) {
                current = &mut current.children[pos];
            } else {
                let parent_path = current.path.join(&comp);
                current.children.push(Node::new(comp.clone(), parent_path, true));
                let last_idx = current.children.len() - 1;
                current = &mut current.children[last_idx];
            }
        }
    }
}

fn sort_tree(node: &mut Node) {
    node.children.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.cmp(&b.name),
        }
    });
    for child in &mut node.children {
        sort_tree(child);
    }
}