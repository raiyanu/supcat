use common::OutputFormat;
use core::{walk, WalkOptions};

fn print_help() {
    println!("supcat - Cross-platform terminal AI context preparer");
    println!();
    println!("Usage:");
    println!("  supcat [options] [path]");
    println!();
    println!("Options:");
    println!("  -h, --help          Show this help text");
    println!("  -a, --hidden        Include hidden files and directories");
    println!("  --no-gitignore      Do not respect .gitignore rules");
    println!("  --symlinks          Follow symlinks");
    println!("  --max-depth <depth> Limit directory traversal depth");
    println!("  --format <format>   Initial formatting option: plain, markdown, xml, json (default: plain)");
    println!("  -c, --clipboard     Copy context to clipboard instead of stdout");
}

fn main() {
    let mut root = ".".to_string();
    let mut show_hidden = false;
    let mut respect_gitignore = true;
    let mut follow_symlinks = false;
    let mut max_depth = None;
    let mut format = OutputFormat::Plain;
    let mut clipboard = false;

    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-a" | "--hidden" => {
                show_hidden = true;
            }
            "--no-gitignore" => {
                respect_gitignore = false;
            }
            "--symlinks" => {
                follow_symlinks = true;
            }
            "--max-depth" => {
                if i + 1 < args.len() {
                    if let Ok(d) = args[i+1].parse::<usize>() {
                        max_depth = Some(d);
                    }
                    i += 1;
                }
            }
            "--format" => {
                if i + 1 < args.len() {
                    format = match args[i+1].to_lowercase().as_str() {
                        "markdown" | "md" => OutputFormat::Markdown,
                        "xml" => OutputFormat::Xml,
                        "json" => OutputFormat::Json,
                        _ => OutputFormat::Plain,
                    };
                    i += 1;
                }
            }
            "-c" | "--clipboard" => {
                clipboard = true;
            }
            val if !val.starts_with('-') => {
                root = val.to_string();
            }
            _ => {}
        }
        i += 1;
    }

    let walk_options = WalkOptions {
        show_hidden,
        respect_gitignore,
        follow_symlinks,
        max_depth,
    };

    match walk(&root, &walk_options) {
        Ok(mut tree) => {
            tree.expanded = true;
            if let Ok(Some((final_tree, final_format))) = tui::run(tree, format) {
                let context = output::format_context(&final_tree, final_format);
                if !context.is_empty() {
                    if clipboard {
                        match copy_to_clipboard(&context) {
                            Ok(_) => eprintln!("Successfully copied context to clipboard."),
                            Err(_) => {
                                eprintln!("Warning: Failed to copy to clipboard. Printing to stdout instead.");
                                print!("{}", context);
                            }
                        }
                    } else {
                        print!("{}", context);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Error walking directory: {err}");
        }
    }
}

fn copy_to_clipboard(text: &str) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use std::process::{Command, Stdio};
        use std::io::Write;
        let mut child = Command::new("pbcopy")
            .stdin(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    #[cfg(target_os = "linux")]
    {
        use std::process::{Command, Stdio};
        use std::io::Write;
        if let Ok(mut child) = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()?;
            return Ok(());
        }
        if let Ok(mut child) = Command::new("xsel")
            .args(&["--clipboard", "--input"])
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()?;
            return Ok(());
        }
        if let Ok(mut child) = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text.as_bytes())?;
            }
            child.wait()?;
            return Ok(());
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::{Command, Stdio};
        use std::io::Write;
        let mut child = Command::new("clip")
            .stdin(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        return Ok(());
    }

    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "No clipboard utility found"))
}