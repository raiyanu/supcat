use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct VisibleNode {
    pub name: String,
    pub path: PathBuf,
    pub depth: usize,
    pub is_dir: bool,
    pub expanded: bool,
    pub selected: bool,
}