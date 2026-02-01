use colored::*;
use std::process::Command;

pub fn execute(filter: Option<String>) -> Result<(), String> {
    println!("{} Listening Ports", "🔌".green().bold());
    println!();

    // Try `ss` first (modern replacement for netstat)
    let output = Command::new("ss")
        .args(&["-tuln"])
        .output();

    let result = if let Ok(r) = output {
        if r.status.success() {
            Some(String::from_utf8_lossy(&r.stdout).to_string())
        } else {
            None
        }
    } else {
        None
    };

    // Fallback to netstat
    let result = result.or_else(|| {
        Command::new("netstat")
            .args(&["-tuln"])
            .output()
            .ok()
            .and_then(|r| {
                if r.status.success() {
                    Some(String::from_utf8_lossy(&r.stdout).to_string())
                } else {
                    None
                }
            })
    });

    let result = result.ok_or("Failed to run ss or netstat command")?;

    let mut listening_ports = Vec::new();

    for line in result.lines().skip(1) {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let proto = parts[0];
            let local_addr = parts.get(4).unwrap_or(&"");

            // Extract port from address (format: 0.0.0.0:PORT or [::]:PORT)
            if let Some(port_part) = local_addr.rsplit(':').next() {
                if let Some(ref filter_port) = filter {
                    if !port_part.contains(filter_port) {
                        continue;
                    }
                }
                listening_ports.push((proto.to_string(), local_addr.to_string(), port_part.to_string()));
            }
        }
    }

    if listening_ports.is_empty() {
        println!("{} No listening ports found", "ℹ️".yellow());
        return Ok(());
    }

    listening_ports.sort_by(|a, b| {
        a.2.parse::<u16>().unwrap_or(0).cmp(&b.2.parse::<u16>().unwrap_or(0))
    });

    println!("  {:<8} {:<25} {}", "PROTO".bold(), "ADDRESS".bold(), "PORT".bold());
    println!("  {}", "─".repeat(50).dimmed());

    for (proto, addr, port) in listening_ports {
        let proto_colored = if proto.starts_with("tcp") {
            proto.cyan()
        } else {
            proto.yellow()
        };
        println!("  {:<8} {:<25} {}", proto_colored, addr.dimmed(), port.green().bold());
    }

    Ok(())
}
