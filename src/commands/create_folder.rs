use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(paths: Vec<PathBuf>, parents: bool) -> Result<(), String> {
    for path in paths {
        if parents {
            fs::create_dir_all(&path).map_err(|e| {
                format!("Cannot create folder '{}': {}", path.display(), e)
            })?;
        } else {
            fs::create_dir(&path).map_err(|e| {
                format!("Cannot create folder '{}': {}", path.display(), e)
            })?;
        }
        println!("{} Created folder '{}'", "✓".green(), path.display());
    }
    Ok(())
}
