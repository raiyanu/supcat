use common::{Node, VisibleNode};

pub fn flatten(root: &Node) -> Vec<VisibleNode> {
    let mut out = Vec::new();

    visit(root, 0, &mut out);

    out
}

fn visit(node: &Node, depth: usize, out: &mut Vec<VisibleNode>) {
    out.push(VisibleNode {
        name: node.name.clone(),
        path: node.path.clone(),
        depth,
        is_dir: node.is_dir,
        expanded: node.expanded,
        selected: node.selected,
    });

    if node.is_dir && node.expanded {
        for child in &node.children {
            visit(child, depth + 1, out);
        }
    }
}

pub fn flatten_filtered(root: &Node, query: &str) -> Vec<VisibleNode> {
    let mut out = Vec::new();
    let query_lower = query.to_lowercase();
    visit_filtered(root, 0, &query_lower, &mut out);
    out
}

fn matches_query(node: &Node, query: &str) -> bool {
    if node.name.to_lowercase().contains(query) {
        return true;
    }
    node.children.iter().any(|child| matches_query(child, query))
}

fn visit_filtered(node: &Node, depth: usize, query: &str, out: &mut Vec<VisibleNode>) {
    let node_matches = node.name.to_lowercase().contains(query);
    let any_child_matches = node.children.iter().any(|child| matches_query(child, query));

    if !node_matches && !any_child_matches {
        return;
    }

    let is_expanded = node.expanded || any_child_matches;

    out.push(VisibleNode {
        name: node.name.clone(),
        path: node.path.clone(),
        depth,
        is_dir: node.is_dir,
        expanded: is_expanded,
        selected: node.selected,
    });

    if node.is_dir && is_expanded {
        for child in &node.children {
            visit_filtered(child, depth + 1, query, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_flatten_and_filter() {
        let mut root = Node::new("root".to_string(), PathBuf::from("/root"), true);
        root.expanded = true;
        let mut child1 = Node::new("child1.txt".to_string(), PathBuf::from("/root/child1.txt"), false);
        child1.selected = true;
        let mut child2 = Node::new("sub".to_string(), PathBuf::from("/root/sub"), true);
        child2.expanded = true;
        let mut child3 = Node::new("target_file.rs".to_string(), PathBuf::from("/root/sub/target_file.rs"), false);
        child3.selected = true;
        child2.children.push(child3);
        root.children.push(child1);
        root.children.push(child2);

        let flattened = flatten(&root);
        assert_eq!(flattened.len(), 4);
        assert_eq!(flattened[0].name, "root");
        assert_eq!(flattened[1].name, "child1.txt");
        assert_eq!(flattened[1].selected, true);
        assert_eq!(flattened[2].name, "sub");
        assert_eq!(flattened[3].name, "target_file.rs");

        let filtered = flatten_filtered(&root, "target");
        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0].name, "root");
        assert_eq!(filtered[1].name, "sub");
        assert_eq!(filtered[2].name, "target_file.rs");
    }
}