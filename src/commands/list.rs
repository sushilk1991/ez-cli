use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::utils;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    path: PathBuf,
    all: bool,
    details: bool,
    time: bool,
    size: bool,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let entries = fs::read_dir(&path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot open '{}': {}", path.display(), e))
        } else {
            EzError::General(format!("Cannot open '{}': {}", path.display(), e))
        }
    })?;

    let mut items: Vec<_> = entries.filter_map(|e| e.ok()).collect();

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

    let mut json_entries = Vec::new();
    let mut shown = 0;

    for entry in items {
        let name = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        if !all && name_str.starts_with('.') {
            continue;
        }

        let metadata = entry.metadata();
        let is_dir = metadata.as_ref().map(|m| m.is_dir()).unwrap_or(false);
        let file_size = if is_dir { None } else { metadata.as_ref().ok().map(|m| m.len()) };
        let modified = metadata.as_ref().ok().and_then(|m| m.modified().ok()).map(|t| utils::format_time_iso8601(t));

        json_entries.push(serde_json::json!({
            "name": name_str,
            "type": if is_dir { "directory" } else { "file" },
            "size": file_size,
            "modified": modified,
        }));

        if !ctx.json {
            if details {
                let size_str = if is_dir {
                    "-".dimmed().to_string()
                } else {
                    let sz = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
                    utils::format_size(sz).cyan().to_string()
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
    }

    if !ctx.json && !details && shown % 4 != 0 {
        println!();
    }

    Ok(CommandOutput::new("list", serde_json::json!(json_entries)))
}
