use std::env;
use colored::*;

pub fn execute() -> Result<(), String> {
    let current = env::current_dir().map_err(|e| format!("Cannot get current directory: {}", e))?;
    println!("{}", current.display().to_string().cyan());
    Ok(())
}
