use std::fs;
use std::path::{Path, PathBuf};
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(path: PathBuf, depth: usize, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !ctx.json {
        println!("{} {}", "🌲".green(), path.display().to_string().cyan().bold());
        print_tree(&path, "", 0, depth)?;
    }

    let tree_data = build_tree_json(&path, 0, depth)?;
    Ok(CommandOutput::new("tree", tree_data))
}

fn build_tree_json(path: &Path, current_depth: usize, max_depth: usize) -> Result<serde_json::Value, EzError> {
    if current_depth >= max_depth {
        return Ok(serde_json::json!([]));
    }

    let entries = fs::read_dir(path).map_err(|e| {
        EzError::General(format!("Cannot read directory '{}': {}", path.display(), e))
    })?;

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    let mut result = Vec::new();
    for entry in items {
        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = entry.metadata().map(|m| m.is_dir()).unwrap_or(false);

        let children = if is_dir {
            build_tree_json(&entry.path(), current_depth + 1, max_depth)?
        } else {
            serde_json::Value::Null
        };

        let mut node = serde_json::json!({
            "name": name,
            "type": if is_dir { "directory" } else { "file" },
        });
        if is_dir {
            node["children"] = children;
        }
        result.push(node);
    }

    Ok(serde_json::json!(result))
}

fn print_tree(path: &Path, prefix: &str, current_depth: usize, max_depth: usize) -> Result<(), EzError> {
    if current_depth >= max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(path).map_err(|e| {
        EzError::General(format!("Cannot read directory '{}': {}", path.display(), e))
    })?;

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    items.sort_by(|a, b| {
        let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
        let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
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
        let is_dir = entry.metadata().map(|m| m.is_dir()).unwrap_or(false);

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
