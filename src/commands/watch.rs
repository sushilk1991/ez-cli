use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(target: String, interval: u64, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let path = PathBuf::from(&target);
    let is_file = path.exists() && path.is_file();

    if is_file {
        watch_file(path, interval, ctx)
    } else {
        watch_command(target, interval, ctx)
    }
}

fn watch_file(path: PathBuf, interval: u64, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !ctx.json {
        println!("{} Watching {} (Press Ctrl+C to stop)",
            "👁️".cyan(),
            path.display().to_string().yellow().bold()
        );
        println!();
    }

    let mut last_modified = fs::metadata(&path)
        .and_then(|m| m.modified())
        .ok();

    loop {
        thread::sleep(Duration::from_secs(interval));

        let current_modified = fs::metadata(&path)
            .and_then(|m| m.modified())
            .ok();

        if current_modified != last_modified {
            let now = chrono::Local::now();
            if ctx.json {
                println!("{}", serde_json::json!({
                    "event": "changed",
                    "target": path.display().to_string(),
                    "time": now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                }));
            } else {
                println!("{} {} File changed!",
                    now.format("%H:%M:%S").to_string().dimmed(),
                    "🔄".green()
                );
            }
            last_modified = current_modified;
        }
    }
}

fn watch_command(command: String, interval: u64, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if !ctx.json {
        println!("{} Watching command: {} (Press Ctrl+C to stop)",
            "👁️".cyan(),
            command.yellow().bold()
        );
        println!();
    }

    let mut last_output = String::new();

    loop {
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| EzError::General(format!("Failed to run command: {}", e)))?;

        let current_output = String::from_utf8_lossy(&output.stdout).to_string();

        if current_output != last_output {
            let now = chrono::Local::now();
            if ctx.json {
                println!("{}", serde_json::json!({
                    "event": "changed",
                    "target": command,
                    "time": now.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
                    "output": current_output,
                }));
            } else {
                println!("\n{} {} Output changed:",
                    now.format("%H:%M:%S").to_string().dimmed(),
                    "🔄".green()
                );
                println!("{}", "─".repeat(60).dimmed());
                println!("{}", current_output);
            }
            last_output = current_output;
        }

        thread::sleep(Duration::from_secs(interval));
    }
}
