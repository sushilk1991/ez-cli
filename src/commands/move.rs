use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(from: PathBuf, to: PathBuf) -> Result<(), String> {
    if !from.exists() {
        return Err(format!("Source '{}' does not exist", from.display()));
    }

    fs::rename(&from, &to).map_err(|e| {
        format!("Cannot move '{}': {}", from.display(), e)
    })?;

    println!("{} Moved '{}' to '{}'", "✓".green(), from.display(), to.display());
    Ok(())
}
