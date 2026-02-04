use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(path: PathBuf, detailed: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !path.exists() {
        return Err(EzError::NotFound(format!("Path '{}' does not exist", path.display())));
    }

    if !path.is_dir() {
        let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        if !ctx.json {
            println!("{} {}", crate::utils::format_size(size).cyan(), path.display());
        }
        return Ok(CommandOutput::new("size", serde_json::json!({
            "path": path.display().to_string(),
            "total_size": size,
            "files": 1,
            "directories": 0,
        })));
    }

    if detailed {
        show_detailed(&path, ctx)
    } else {
        show_total(&path, ctx)
    }
}

fn show_total(path: &PathBuf, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_size += metadata.len();
                file_count += 1;
            } else if metadata.is_dir() && entry.path() != path.as_path() {
                dir_count += 1;
            }
        }
    }

    if !ctx.json {
        println!("{} Total size: {}", "📦".cyan(), crate::utils::format_size(total_size).bold());
        println!("   {} files, {} folders", file_count, dir_count);
    }

    Ok(CommandOutput::new("size", serde_json::json!({
        "path": path.display().to_string(),
        "total_size": total_size,
        "files": file_count,
        "directories": dir_count,
    })))
}

fn show_detailed(path: &PathBuf, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let mut entries_data: Vec<serde_json::Value> = vec![];

    for entry in fs::read_dir(path).map_err(|e| EzError::General(format!("Cannot read directory: {}", e)))? {
        let entry = entry.map_err(|e| EzError::General(format!("Read error: {}", e)))?;
        let entry_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = if is_dir {
            calculate_dir_size(&entry_path)
        } else {
            metadata.map(|m| m.len()).unwrap_or(0)
        };

        entries_data.push(serde_json::json!({
            "name": name,
            "type": if is_dir { "directory" } else { "file" },
            "size": size,
        }));
    }

    entries_data.sort_by(|a, b| {
        let a_size = a["size"].as_u64().unwrap_or(0);
        let b_size = b["size"].as_u64().unwrap_or(0);
        b_size.cmp(&a_size)
    });

    if !ctx.json {
        println!("{:<10} {}", "Size".bold().underline(), "Name".bold().underline());
        for item in &entries_data {
            let name = item["name"].as_str().unwrap_or("");
            let size = item["size"].as_u64().unwrap_or(0);
            let is_dir = item["type"].as_str() == Some("directory");
            let size_str = crate::utils::format_size(size);
            let icon = if is_dir { "📁" } else { "📄" };
            let name_colored = if is_dir { name.blue().bold() } else { name.normal() };
            println!("{:>10} {} {}", size_str.cyan(), icon, name_colored);
        }
    }

    Ok(CommandOutput::new("size", serde_json::json!(entries_data)))
}

fn calculate_dir_size(path: &PathBuf) -> u64 {
    let mut total: u64 = 0;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total += metadata.len();
            }
        }
    }
    total
}
