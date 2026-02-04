use std::process::Command;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let output = Command::new("df")
        .args(&["-h"])
        .output()
        .map_err(|e| EzError::General(format!("Failed to run df: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        return Err(EzError::General("No disk information available".to_string()));
    }

    let mut filesystems = Vec::new();

    for line in &lines[1..] {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            let fs_type = parts[0];
            if !fs_type.starts_with("/dev/") && fs_type != "Filesystem" {
                continue;
            }
            filesystems.push(serde_json::json!({
                "filesystem": parts[0],
                "size": parts[1],
                "used": parts[2],
                "available": parts[3],
                "use_percent": parts[4],
                "mounted_on": parts[5],
            }));
        }
    }

    if !ctx.json {
        println!("{}", lines[0].bold().underline());
        for line in &lines[1..] {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let fs_type = parts[0];
                if !fs_type.starts_with("/dev/") && fs_type != "Filesystem" {
                    continue;
                }
                let filesystem = parts[0].cyan();
                let size = parts[1].yellow();
                let used = parts[2];
                let available = parts[3].green();
                let percent = parts[4];
                let mounted = parts[5];
                let percent_colored = if percent.starts_with("9") || percent == "100%" {
                    percent.red().bold()
                } else if percent.starts_with("8") {
                    percent.yellow()
                } else {
                    percent.normal()
                };
                println!("{:<20} {:>8} {:>8} {:>8} {:>6} {}",
                    filesystem, size, used, available, percent_colored, mounted);
            }
        }
    }

    Ok(CommandOutput::new("space", serde_json::json!(filesystems)))
}
