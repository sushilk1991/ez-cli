use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::utils;

pub fn execute(
    path: PathBuf,
    all: bool,
    details: bool,
    time: bool,
    size: bool,
) -> Result<(), String> {
    let entries = fs::read_dir(&path).map_err(|e| {
        format!("Cannot open '{}': {}", path.display(), e)
    })?;

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();

    // Sort
    if time {
        items.sort_by(|a, b| {
            let a_time = a.metadata().and_then(|m| m.modified()).ok();
            let b_time = b.metadata().and_then(|m| m.modified()).ok();
            b_time.cmp(&a_time)
        });
    } else if size {
        items.sort_by(|a, b| {
            let a_size = a.metadata().map(|m| m.len()).unwrap_or(0);
            let b_size = b.metadata().map(|m| m.len()).unwrap_or(0);
            b_size.cmp(&a_size)
        });
    } else {
        items.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    }

    let mut shown = 0;

    for entry in items {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // Skip hidden files unless --all
        if !all && name_str.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);

        if details {
            let size_str = if is_dir {
                "-".dimmed().to_string()
            } else {
                let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                utils::format_size(size).cyan().to_string()
            };

            let time_str = metadata
                .as_ref()
                .ok()
                .and_then(|m| m.modified().ok())
                .map(|t| utils::format_time(t))
                .unwrap_or_else(|| "-".dimmed().to_string());

            let type_icon = if is_dir { "📁" } else { "📄" };
            let name_colored = if is_dir {
                name_str.blue().bold()
            } else {
                name_str.normal()
            };

            println!("{} {:>10} {:>12} {}", type_icon, size_str, time_str.dimmed(), name_colored);
        } else {
            let name_colored = if is_dir {
                name_str.blue().bold()
            } else {
                name_str.normal()
            };
            print!("{}  ", name_colored);
            shown += 1;
            if shown % 4 == 0 {
                println!();
            }
        }
    }

    if !details && shown % 4 != 0 {
        println!();
    }

    Ok(())
}
