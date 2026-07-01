use common::{Node, OutputFormat};
use core::{walk, WalkOptions};
use std::fs;

#[test]
fn test_entire_pipeline_with_gitignore_and_formatting() {
    let temp_dir = std::env::temp_dir().join("supcat_integration_test_dir");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();
    let temp_dir = temp_dir.canonicalize().unwrap();

    let src_dir = temp_dir.join("src");
    let target_dir = temp_dir.join("target");
    let hidden_dir = temp_dir.join(".hidden");
    let git_dir = temp_dir.join(".git");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&target_dir).unwrap();
    fs::create_dir_all(&hidden_dir).unwrap();
    fs::create_dir_all(&git_dir).unwrap();

    fs::write(temp_dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
    fs::write(src_dir.join("main.rs"), "fn main() {}").unwrap();
    fs::write(src_dir.join("test.log"), "some logs").unwrap();
    fs::write(target_dir.join("debug_binary"), "binary data").unwrap();
    fs::write(hidden_dir.join("secret.txt"), "shh").unwrap();

    fs::write(temp_dir.join(".gitignore"), "*.log\ntarget/").unwrap();

    let walk_options = WalkOptions {
        show_hidden: false,
        respect_gitignore: true,
        follow_symlinks: false,
        max_depth: None,
    };

    let tree = walk(&temp_dir, &walk_options).expect("failed to walk temp directory");

    fn find_child<'a>(node: &'a Node, name: &str) -> Option<&'a Node> {
        node.children.iter().find(|c| c.name == name)
    }

    assert!(find_child(&tree, "Cargo.toml").is_some());
    assert!(find_child(&tree, "src").is_some());

    let src_node = find_child(&tree, "src").unwrap();
    assert!(find_child(src_node, "main.rs").is_some());

    assert!(find_child(&tree, "target").is_none());
    assert!(find_child(&tree, ".hidden").is_none());
    assert!(find_child(src_node, "test.log").is_none());

    let walk_options_all = WalkOptions {
        show_hidden: true,
        respect_gitignore: false,
        follow_symlinks: false,
        max_depth: None,
    };

    let tree_all = walk(&temp_dir, &walk_options_all).expect("failed to walk temp directory with all files");
    assert!(find_child(&tree_all, "target").is_some());
    assert!(find_child(&tree_all, ".hidden").is_some());

    let src_node_all = find_child(&tree_all, "src").unwrap();
    assert!(find_child(src_node_all, "test.log").is_some());

    let mut tree_to_format = tree.clone();
    let src_node_mut = tree_to_format.children.iter_mut().find(|c| c.name == "src").unwrap();
    let main_rs_node = src_node_mut.children.iter_mut().find(|c| c.name == "main.rs").unwrap();
    main_rs_node.selected = true;

    let plain_output = output::format_context(&tree_to_format, OutputFormat::Plain);
    assert!(plain_output.contains("===== src/main.rs ====="));
    assert!(plain_output.contains("fn main() {}"));

    let md_output = output::format_context(&tree_to_format, OutputFormat::Markdown);
    assert!(md_output.contains("## src/main.rs"));
    assert!(md_output.contains("```rs\nfn main() {}\n```"));

    let _ = fs::remove_dir_all(&temp_dir);
}
