pub mod node;
pub mod visible_node;

pub use node::Node;
pub use visible_node::VisibleNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Plain,
    Markdown,
    Xml,
    Json,
}

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;