use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(paths: Vec<PathBuf>, recursive: bool, force: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let mut removed = Vec::new();
    let mut skipped = Vec::new();

    for path in paths {
        if !path.exists() {
            if force {
                continue;
            }
            return Err(EzError::NotFound(format!("'{}' does not exist", path.display())));
        }

        let is_dir = path.is_dir();

        if is_dir && !recursive {
            return Err(EzError::InvalidArgs(format!(
                "'{}' is a folder. Use --recursive to remove folders",
                path.display()
            )));
        }

        // Dry-run: report but don't remove
        if ctx.dry_run {
            let path_str = path.display().to_string();
            if !ctx.json {
                println!("{} Would remove '{}'", "~".dimmed(), path_str);
            }
            removed.push(path_str);
            continue;
        }

        // Confirmation for directories
        if is_dir && !force {
            if !ctx.should_confirm() {
                return Err(EzError::Cancelled(format!(
                    "Cannot confirm removal of '{}' in non-interactive mode. Use --yes or --force.",
                    path.display()
                )));
            }
            print!("Remove folder '{}' and all contents? [y/N] ", path.display());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if !input.trim().eq_ignore_ascii_case("y") {
                skipped.push(path.display().to_string());
                if !ctx.json {
                    println!("Skipped '{}'", path.display());
                }
                continue;
            }
        }

        if is_dir {
            fs::remove_dir_all(&path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    EzError::PermissionDenied(format!("Cannot remove: {}", e))
                } else {
                    EzError::General(format!("Cannot remove: {}", e))
                }
            })?;
        } else {
            fs::remove_file(&path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::PermissionDenied {
                    EzError::PermissionDenied(format!("Cannot remove: {}", e))
                } else {
                    EzError::General(format!("Cannot remove: {}", e))
                }
            })?;
        }

        removed.push(path.display().to_string());
        if !ctx.json {
            println!("{} Removed '{}'", "✓".green(), path.display());
        }
    }

    let mut output = CommandOutput::new("remove", serde_json::json!({
        "removed": removed,
        "skipped": skipped,
    }));
    if ctx.dry_run {
        output.metadata = Some(serde_json::json!({ "dry_run": true }));
    }
    Ok(output)
}
