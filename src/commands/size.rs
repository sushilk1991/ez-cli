use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;

pub fn execute(path: PathBuf, detailed: bool) -> Result<(), String> {
    if !path.exists() {
        return Err(format!("Path '{}' does not exist", path.display()));
    }

    if !path.is_dir() {
        let size = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        println!("{} {}", crate::utils::format_size(size).cyan(), path.display());
        return Ok(());
    }

    if detailed {
        show_detailed(&path)?;
    } else {
        show_total(&path)?;
    }

    Ok(())
}

fn show_total(path: &PathBuf) -> Result<(), String> {
    let mut total_size: u64 = 0;
    let mut file_count: u64 = 0;
    let mut dir_count: u64 = 0;

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                total_size += metadata.len();
                file_count += 1;
            } else if metadata.is_dir() && entry.path() != path {
                dir_count += 1;
            }
        }
    }

    println!("{} Total size: {}", "📦".cyan(), crate::utils::format_size(total_size).bold());
    println!("   {} files, {} folders", file_count, dir_count);

    Ok(())
}

fn show_detailed(path: &PathBuf) -> Result<(), String> {
    let mut entries: Vec<(PathBuf, u64, bool)> = vec![];

    for entry in fs::read_dir(path).map_err(|e| format!("Cannot read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Read error: {}", e))?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        
        if name.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let size = if is_dir {
            calculate_dir_size(&path)
        } else {
            metadata.map(|m| m.len()).unwrap_or(0)
        };

        entries.push((path, size, is_dir));
    }

    entries.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:<10} {}", "Size".bold().underline(), "Name".bold().underline());
    for (path, size, is_dir) in entries {
        let name = path.file_name().unwrap().to_string_lossy();
        let size_str = crate::utils::format_size(size);
        let icon = if is_dir { "📁" } else { "📄" };
        let name_colored = if is_dir { name.blue().bold() } else { name.normal() };
        println!("{:>10} {} {}", size_str.cyan(), icon, name_colored);
    }

    Ok(())
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
