use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(file: PathBuf, if_not_exists: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let metadata = fs::metadata(&file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot access '{}': {}", file.display(), e))
        } else if e.kind() == std::io::ErrorKind::PermissionDenied {
            EzError::PermissionDenied(format!("Cannot access '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot access '{}': {}", file.display(), e))
        }
    })?;

    let mut permissions = metadata.permissions();
    let current_mode = permissions.mode();

    // Check if already executable
    if if_not_exists && (current_mode & 0o111) != 0 {
        if !ctx.json {
            println!("{} Skipped '{}' (already executable)", "~".dimmed(), file.display());
        }
        return Ok(CommandOutput::new("make-runnable", serde_json::json!({
            "file": file.display().to_string(),
            "already_executable": true,
        })).with_metadata(serde_json::json!({ "skipped": true })));
    }

    let new_mode = current_mode | 0o111;
    permissions.set_mode(new_mode);

    fs::set_permissions(&file, permissions).map_err(|e| {
        EzError::PermissionDenied(format!("Cannot change permissions: {}", e))
    })?;

    if !ctx.json {
        println!("{} Made '{}' executable", "✓".green(), file.display().to_string().cyan());
    }

    Ok(CommandOutput::new("make-runnable", serde_json::json!({
        "file": file.display().to_string(),
        "mode": format!("{:o}", new_mode & 0o777),
    })))
}
