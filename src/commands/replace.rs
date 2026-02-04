use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    old: String,
    new: String,
    file: PathBuf,
    all: bool,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot read '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot read '{}': {}", file.display(), e))
        }
    })?;

    let (new_contents, count) = if all {
        let count = contents.matches(&old).count();
        (contents.replace(&old, &new), count)
    } else {
        if let Some(pos) = contents.find(&old) {
            let mut result = contents.clone();
            result.replace_range(pos..pos + old.len(), &new);
            (result, 1)
        } else {
            (contents.clone(), 0)
        }
    };

    if count == 0 {
        if !ctx.json {
            println!("{} No matches found for '{}'", "ℹ️".yellow(), old.yellow());
        }
        return Ok(CommandOutput::new("replace", serde_json::json!({
            "file": file.display().to_string(),
            "replacements": 0,
        })));
    }

    if ctx.dry_run {
        if !ctx.json {
            println!("{} Would replace {} occurrence(s) of '{}' with '{}' in {}",
                "~".dimmed(), count, old.yellow(), new.green(), file.display().to_string().cyan());
        }
        return Ok(CommandOutput::new("replace", serde_json::json!({
            "file": file.display().to_string(),
            "replacements": count,
        })).with_metadata(serde_json::json!({ "dry_run": true })));
    }

    fs::write(&file, new_contents).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            EzError::PermissionDenied(format!("Cannot write to '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot write to '{}': {}", file.display(), e))
        }
    })?;

    if !ctx.json {
        println!("{} Replaced {} occurrence(s) of '{}' with '{}' in {}",
            "✓".green(),
            count.to_string().cyan().bold(),
            old.yellow(),
            new.green(),
            file.display().to_string().cyan()
        );
    }

    Ok(CommandOutput::new("replace", serde_json::json!({
        "file": file.display().to_string(),
        "replacements": count,
    })))
}
