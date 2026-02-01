use std::fs;
use std::path::{Path, PathBuf};
use colored::*;

pub fn execute(path: PathBuf, depth: usize) -> Result<(), String> {
    println!("{} {}", "🌲".green(), path.display().to_string().cyan().bold());
    print_tree(&path, "", 0, depth)?;
    Ok(())
}

fn print_tree(path: &Path, prefix: &str, current_depth: usize, max_depth: usize) -> Result<(), String> {
    if current_depth >= max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(path).map_err(|e| {
        format!("Cannot read directory '{}': {}", path.display(), e)
    })?;

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
        
        // Directories first, then alphabetical
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let count = items.len();
    for (i, entry) in items.iter().enumerate() {
        let is_last = i == count - 1;
        let connector = if is_last { "└──" } else { "├──" };
        let extension = if is_last { "    " } else { "│   " };

        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        
        let metadata = entry.metadata().ok();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        let display_name = if is_dir {
            format!("{}/", name_str).blue().bold()
        } else {
            name_str.normal()
        };

        println!("{}{} {}", prefix, connector.dimmed(), display_name);

        if is_dir {
            let new_prefix = format!("{}{}", prefix, extension);
            print_tree(&entry.path(), &new_prefix, current_depth + 1, max_depth)?;
        }
    }

    Ok(())
}
