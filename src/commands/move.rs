use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(from: PathBuf, to: PathBuf, if_not_exists: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !from.exists() {
        return Err(EzError::NotFound(format!("Source '{}' does not exist", from.display())));
    }

    if if_not_exists && to.exists() {
        if !ctx.json {
            println!("{} Skipped move (destination '{}' already exists)", "~".dimmed(), to.display());
        }
        return Ok(CommandOutput::new("move", serde_json::json!({
            "from": from.display().to_string(),
            "to": to.display().to_string(),
        })).with_metadata(serde_json::json!({ "skipped": true })));
    }

    if ctx.dry_run {
        if !ctx.json {
            println!("{} Would move '{}' to '{}'", "~".dimmed(), from.display(), to.display());
        }
        return Ok(CommandOutput::new("move", serde_json::json!({
            "from": from.display().to_string(),
            "to": to.display().to_string(),
        })).with_metadata(serde_json::json!({ "dry_run": true })));
    }

    fs::rename(&from, &to).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            EzError::PermissionDenied(format!("Cannot move '{}': {}", from.display(), e))
        } else {
            EzError::General(format!("Cannot move '{}': {}", from.display(), e))
        }
    })?;

    if !ctx.json {
        println!("{} Moved '{}' to '{}'", "✓".green(), from.display(), to.display());
    }

    Ok(CommandOutput::new("move", serde_json::json!({
        "from": from.display().to_string(),
        "to": to.display().to_string(),
    })))
}
