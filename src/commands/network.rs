use colored::*;
use std::process::Command;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let output = Command::new("ip").arg("addr").output();

    if let Ok(result) = output {
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let interfaces = parse_ip_addr_json(&stdout);

            if !ctx.json {
                println!("{} Network Interfaces", "🌐".green().bold());
                println!();
                parse_ip_addr_pretty(&stdout);
            }

            return Ok(CommandOutput::new("network", serde_json::json!(interfaces)));
        }
    }

    let output = Command::new("ifconfig")
        .output()
        .map_err(|e| EzError::General(format!("Failed to run network command: {}", e)))?;

    if !output.status.success() {
        return Err(EzError::General("Network command failed".to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !ctx.json {
        println!("{} Network Interfaces", "🌐".green().bold());
        println!();
        println!("{}", stdout);
    }

    Ok(CommandOutput::new("network", serde_json::json!({ "raw": stdout.to_string() })))
}

fn parse_ip_addr_json(output: &str) -> Vec<serde_json::Value> {
    let mut interfaces = Vec::new();
    let mut current_name = String::new();
    let mut current_state = String::new();
    let mut ipv4s = Vec::new();
    let mut ipv6s = Vec::new();

    for line in output.lines() {
        if !line.starts_with(' ') && !line.is_empty() {
            if !current_name.is_empty() {
                interfaces.push(serde_json::json!({
                    "name": current_name,
                    "state": current_state,
                    "ipv4": ipv4s,
                    "ipv6": ipv6s,
                }));
                ipv4s = Vec::new();
                ipv6s = Vec::new();
            }
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                current_name = parts[1].trim().to_string();
                current_state = if line.contains("state UP") { "UP".to_string() } else { "DOWN".to_string() };
            }
        } else if line.contains("inet ") && !current_name.is_empty() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet" {
                ipv4s.push(parts[1].to_string());
            }
        } else if line.contains("inet6 ") && !current_name.is_empty() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet6" {
                ipv6s.push(parts[1].to_string());
            }
        }
    }
    if !current_name.is_empty() {
        interfaces.push(serde_json::json!({
            "name": current_name,
            "state": current_state,
            "ipv4": ipv4s,
            "ipv6": ipv6s,
        }));
    }
    interfaces
}

fn parse_ip_addr_pretty(output: &str) {
    let mut current_interface = String::new();

    for line in output.lines() {
        if !line.starts_with(' ') && !line.is_empty() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                current_interface = parts[1].trim().to_string();
                let state = if line.contains("state UP") { "UP".green() } else { "DOWN".red() };
                println!("  {} {} [{}]", "🔌".cyan(), current_interface.bold(), state);
            }
        } else if line.contains("inet ") && !current_interface.is_empty() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet" {
                println!("     {} {}", "IPv4:".dimmed(), parts[1].yellow());
            }
        } else if line.contains("inet6 ") && !current_interface.is_empty() {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "inet6" {
                println!("     {} {}", "IPv6:".dimmed(), parts[1].cyan());
            }
        }
    }
}
