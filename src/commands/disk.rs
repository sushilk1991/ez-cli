use colored::*;
use std::process::Command;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let output = Command::new("iostat")
        .args(&["-x", "1", "2"])
        .output();

    if let Ok(result) = output {
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let devices = parse_iostat_json(&stdout);

            if !ctx.json {
                println!("{} Disk I/O Statistics", "💾".green().bold());
                println!();
                parse_iostat_pretty(&stdout);
            }

            return Ok(CommandOutput::new("disk", serde_json::json!({ "source": "iostat", "devices": devices })));
        }
    }

    // Fallback to df
    let output = Command::new("df")
        .args(&["-h"])
        .output()
        .map_err(|e| EzError::General(format!("Failed to run df: {}", e)))?;

    if !output.status.success() {
        return Err(EzError::General("df command failed".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let filesystems = parse_df_json(&stdout);

    if !ctx.json {
        println!("{} Disk I/O Statistics", "💾".green().bold());
        println!();
        println!("{} iostat not found, showing disk usage instead:", "ℹ️".yellow());
        println!();
        for (i, line) in stdout.lines().enumerate() {
            if i == 0 {
                println!("{}", line.bold());
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(CommandOutput::new("disk", serde_json::json!({ "source": "df", "filesystems": filesystems })))
}

fn parse_iostat_json(output: &str) -> Vec<serde_json::Value> {
    let mut devices = Vec::new();
    let mut in_device_section = false;

    for line in output.lines() {
        if line.contains("Device") && line.contains("r/s") {
            in_device_section = true;
            continue;
        }
        if in_device_section && !line.trim().is_empty() && !line.starts_with("avg") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                devices.push(serde_json::json!({
                    "device": parts[0],
                    "reads_per_sec": parts.get(3).unwrap_or(&"0"),
                    "writes_per_sec": parts.get(4).unwrap_or(&"0"),
                    "read_kb_per_sec": parts.get(5).unwrap_or(&"0"),
                    "write_kb_per_sec": parts.get(6).unwrap_or(&"0"),
                }));
            }
        }
        if line.is_empty() {
            in_device_section = false;
        }
    }
    devices
}

fn parse_iostat_pretty(output: &str) {
    let mut in_device_section = false;
    let mut header_printed = false;

    for line in output.lines() {
        if line.contains("Device") && line.contains("r/s") {
            in_device_section = true;
            if !header_printed {
                println!("  {:<12} {:<8} {:<8} {:<10} {:<10}",
                    "Device".bold(), "r/s".bold(), "w/s".bold(), "rkB/s".bold(), "wkB/s".bold());
                println!("  {}", "─".repeat(60).dimmed());
                header_printed = true;
            }
            continue;
        }
        if in_device_section && !line.trim().is_empty() && !line.starts_with("avg") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                println!("  {:<12} {:<8} {:<8} {:<10} {:<10}",
                    parts[0].cyan(),
                    parts[3].yellow(), parts[4].yellow(),
                    parts[5].green(), parts.get(6).unwrap_or(&"0").green());
            }
        }
        if line.is_empty() {
            in_device_section = false;
        }
    }
    if !header_printed {
        println!("{}", output);
    }
}

fn parse_df_json(output: &str) -> Vec<serde_json::Value> {
    let mut filesystems = Vec::new();
    for line in output.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
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
    filesystems
}
