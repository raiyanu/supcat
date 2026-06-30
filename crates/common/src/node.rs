use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub expanded: bool,
    pub selected: bool,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, path: PathBuf, is_dir: bool) -> Self {
        Self {
            name,
            path,
            is_dir,
            expanded: false,
            selected: false,
            children: Vec::new(),
        }
    }
}