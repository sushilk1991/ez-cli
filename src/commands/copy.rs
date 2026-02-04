use std::fs;
use std::path::PathBuf;
use colored::*;
use std::io::Write;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(from: PathBuf, to: PathBuf, recursive: bool, progress: bool, if_not_exists: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !from.exists() {
        return Err(EzError::NotFound(format!("Source '{}' does not exist", from.display())));
    }

    let is_dir = from.is_dir();

    if is_dir && !recursive {
        return Err(EzError::InvalidArgs(format!(
            "'{}' is a folder. Use --recursive to copy folders",
            from.display()
        )));
    }

    if if_not_exists && to.exists() {
        if !ctx.json {
            println!("{} Skipped copy (destination '{}' already exists)", "~".dimmed(), to.display());
        }
        return Ok(CommandOutput::new("copy", serde_json::json!({
            "from": from.display().to_string(),
            "to": to.display().to_string(),
        })).with_metadata(serde_json::json!({ "skipped": true })));
    }

    if is_dir {
        copy_dir(&from, &to, progress, ctx)?;
    } else {
        copy_file(&from, &to, progress, ctx)?;
    }

    if !ctx.json {
        println!("{} Copied '{}' to '{}'", "✓".green(), from.display(), to.display());
    }

    Ok(CommandOutput::new("copy", serde_json::json!({
        "from": from.display().to_string(),
        "to": to.display().to_string(),
    })))
}

fn copy_file(from: &PathBuf, to: &PathBuf, progress: bool, ctx: &CommandContext) -> Result<(), EzError> {
    if progress && !ctx.json {
        let size = fs::metadata(from).map(|m| m.len()).unwrap_or(0);
        let size_str = crate::utils::format_size(size);
        print!("Copying {}... ", size_str.dimmed());
        std::io::stdout().flush().unwrap();
    }

    fs::copy(from, to).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            EzError::PermissionDenied(format!("Copy failed: {}", e))
        } else {
            EzError::General(format!("Copy failed: {}", e))
        }
    })?;

    if progress && !ctx.json {
        println!("{}", "done".green());
    }

    Ok(())
}

fn copy_dir(from: &PathBuf, to: &PathBuf, progress: bool, ctx: &CommandContext) -> Result<(), EzError> {
    fs::create_dir_all(to).map_err(|e| EzError::General(format!("Cannot create directory: {}", e)))?;

    for entry in fs::read_dir(from).map_err(|e| EzError::General(format!("Cannot read directory: {}", e)))? {
        let entry = entry.map_err(|e| EzError::General(format!("Read error: {}", e)))?;
        let from_path = entry.path();
        let file_name = entry.file_name();
        let to_path = to.join(&file_name);

        if from_path.is_dir() {
            copy_dir(&from_path, &to_path, progress, ctx)?;
        } else {
            copy_file(&from_path, &to_path, progress, ctx)?;
        }
    }

    Ok(())
}
