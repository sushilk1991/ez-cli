use colored::*;
use std::process::Command;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(filter: Option<String>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let output = Command::new("ss").args(&["-tuln"]).output();

    let result = if let Ok(r) = output {
        if r.status.success() { Some(String::from_utf8_lossy(&r.stdout).to_string()) } else { None }
    } else { None };

    let result = result.or_else(|| {
        Command::new("netstat").args(&["-tuln"]).output().ok().and_then(|r| {
            if r.status.success() { Some(String::from_utf8_lossy(&r.stdout).to_string()) } else { None }
        })
    });

    let result = result.ok_or(EzError::General("Failed to run ss or netstat command".to_string()))?;

    let mut listening_ports = Vec::new();

    for line in result.lines().skip(1) {
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let proto = parts[0];
            let local_addr = parts.get(4).unwrap_or(&"");
            if let Some(port_part) = local_addr.rsplit(':').next() {
                if let Some(ref filter_port) = filter {
                    if !port_part.contains(filter_port) { continue; }
                }
                listening_ports.push((proto.to_string(), local_addr.to_string(), port_part.to_string()));
            }
        }
    }

    listening_ports.sort_by(|a, b| {
        a.2.parse::<u16>().unwrap_or(0).cmp(&b.2.parse::<u16>().unwrap_or(0))
    });

    if !ctx.json {
        println!("{} Listening Ports", "🔌".green().bold());
        println!();
        if listening_ports.is_empty() {
            println!("{} No listening ports found", "ℹ️".yellow());
        } else {
            println!("  {:<8} {:<25} {}", "PROTO".bold(), "ADDRESS".bold(), "PORT".bold());
            println!("  {}", "─".repeat(50).dimmed());
            for (proto, addr, port) in &listening_ports {
                let proto_colored = if proto.starts_with("tcp") { proto.cyan() } else { proto.yellow() };
                println!("  {:<8} {:<25} {}", proto_colored, addr.dimmed(), port.green().bold());
            }
        }
    }

    let json_ports: Vec<_> = listening_ports.iter().map(|(proto, addr, port)| {
        serde_json::json!({ "protocol": proto, "address": addr, "port": port })
    }).collect();

    Ok(CommandOutput::new("ports", serde_json::json!(json_ports)))
}
