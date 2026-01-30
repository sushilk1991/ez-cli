use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use colored::*;

pub fn execute(file: PathBuf) -> Result<(), String> {
    let metadata = fs::metadata(&file).map_err(|e| {
        format!("Cannot access '{}': {}", file.display(), e)
    })?;

    let mut permissions = metadata.permissions();
    let current_mode = permissions.mode();
    
    // Add execute permission for owner (0o100)
    let new_mode = current_mode | 0o111;
    permissions.set_mode(new_mode);

    fs::set_permissions(&file, permissions).map_err(|e| {
        format!("Cannot change permissions: {}", e)
    })?;

    println!("{} Made '{}' executable", "✓".green(), file.display().to_string().cyan());
    Ok(())
}
