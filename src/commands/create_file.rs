use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(paths: Vec<PathBuf>, if_not_exists: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let mut created = Vec::new();
    let mut skipped = Vec::new();

    for path in paths {
        if if_not_exists && path.exists() {
            skipped.push(path.display().to_string());
            if !ctx.json {
                println!("{} Skipped '{}' (already exists)", "~".dimmed(), path.display());
            }
            continue;
        }

        fs::File::create(&path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                EzError::PermissionDenied(format!("Cannot create file '{}': {}", path.display(), e))
            } else {
                EzError::General(format!("Cannot create file '{}': {}", path.display(), e))
            }
        })?;
        created.push(path.display().to_string());
        if !ctx.json {
            println!("{} Created file '{}'", "✓".green(), path.display());
        }
    }

    let mut output = CommandOutput::new("create-file", serde_json::json!({
        "created": created,
        "skipped": skipped,
    }));
    if if_not_exists && !skipped.is_empty() {
        output.metadata = Some(serde_json::json!({ "skipped": true }));
    }
    Ok(output)
}
