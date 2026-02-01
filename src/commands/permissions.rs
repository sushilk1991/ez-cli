use std::fs;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use colored::*;

pub fn execute(path: PathBuf) -> Result<(), String> {
    let metadata = fs::metadata(&path).map_err(|e| {
        format!("Cannot read metadata for '{}': {}", path.display(), e)
    })?;

    let permissions = metadata.permissions();
    let mode = permissions.mode();

    // Extract permission bits
    let user_read = mode & 0o400 != 0;
    let user_write = mode & 0o200 != 0;
    let user_exec = mode & 0o100 != 0;
    let group_read = mode & 0o040 != 0;
    let group_write = mode & 0o020 != 0;
    let group_exec = mode & 0o010 != 0;
    let other_read = mode & 0o004 != 0;
    let other_write = mode & 0o002 != 0;
    let other_exec = mode & 0o001 != 0;

    let perm_string = format!(
        "{}{}{}{}{}{}{}{}{}",
        if user_read { "r" } else { "-" },
        if user_write { "w" } else { "-" },
        if user_exec { "x" } else { "-" },
        if group_read { "r" } else { "-" },
        if group_write { "w" } else { "-" },
        if group_exec { "x" } else { "-" },
        if other_read { "r" } else { "-" },
        if other_write { "w" } else { "-" },
        if other_exec { "x" } else { "-" },
    );

    let octal = format!("{:o}", mode & 0o777);

    println!("{} {}", "📋 File:".bold(), path.display().to_string().cyan());
    println!("{} {} ({})", "🔐 Permissions:".bold(), perm_string.yellow(), octal.dimmed());
    println!();
    
    println!("{}", "Breakdown:".bold());
    println!("  {} {} {} {}", 
        "User:".cyan(), 
        if user_read { "✓ read".green() } else { "✗ read".dimmed() },
        if user_write { "✓ write".green() } else { "✗ write".dimmed() },
        if user_exec { "✓ execute".green() } else { "✗ execute".dimmed() }
    );
    println!("  {} {} {} {}", 
        "Group:".cyan(), 
        if group_read { "✓ read".green() } else { "✗ read".dimmed() },
        if group_write { "✓ write".green() } else { "✗ write".dimmed() },
        if group_exec { "✓ execute".green() } else { "✗ execute".dimmed() }
    );
    println!("  {} {} {} {}", 
        "Other:".cyan(), 
        if other_read { "✓ read".green() } else { "✗ read".dimmed() },
        if other_write { "✓ write".green() } else { "✗ write".dimmed() },
        if other_exec { "✓ execute".green() } else { "✗ execute".dimmed() }
    );

    Ok(())
}
