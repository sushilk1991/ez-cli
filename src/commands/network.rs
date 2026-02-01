use colored::*;
use std::process::Command;

pub fn execute() -> Result<(), String> {
    println!("{} Network Interfaces", "🌐".green().bold());
    println!();

    // Try `ip addr` first (more common on modern Linux)
    let output = Command::new("ip")
        .arg("addr")
        .output();

    if let Ok(result) = output {
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            parse_ip_addr(&stdout);
            return Ok(());
        }
    }

    // Fallback to ifconfig
    let output = Command::new("ifconfig")
        .output()
        .map_err(|e| format!("Failed to run network command: {}", e))?;

    if !output.status.success() {
        return Err("Network command failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("{}", stdout);

    Ok(())
}

fn parse_ip_addr(output: &str) {
    let mut current_interface = String::new();
    
    for line in output.lines() {
        if !line.starts_with(' ') && !line.is_empty() {
            // New interface
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                current_interface = parts[1].trim().to_string();
                let state = if line.contains("state UP") {
                    "UP".green()
                } else {
                    "DOWN".red()
                };
                println!("  {} {} [{}]", "🔌".cyan(), current_interface.bold(), state);
            }
        } else if line.contains("inet ") && !current_interface.is_empty() {
            // IP address line
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet" {
                println!("     {} {}", "IPv4:".dimmed(), parts[1].yellow());
            }
        } else if line.contains("inet6 ") && !current_interface.is_empty() {
            // IPv6 address line
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet6" {
                println!("     {} {}", "IPv6:".dimmed(), parts[1].cyan());
            }
        }
    }
}
