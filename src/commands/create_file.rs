use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(paths: Vec<PathBuf>) -> Result<(), String> {
    for path in paths {
        fs::File::create(&path).map_err(|e| {
            format!("Cannot create file '{}': {}", path.display(), e)
        })?;
        println!("{} Created file '{}'", "✓".green(), path.display());
    }
    Ok(())
}
