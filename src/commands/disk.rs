use colored::*;
use std::process::Command;

pub fn execute() -> Result<(), String> {
    println!("{} Disk I/O Statistics", "💾".green().bold());
    println!();

    // Try `iostat` first
    let output = Command::new("iostat")
        .args(&["-x", "1", "2"])
        .output();

    if let Ok(result) = output {
        if result.status.success() {
            let stdout = String::from_utf8_lossy(&result.stdout);
            parse_iostat(&stdout);
            return Ok(());
        }
    }

    // Fallback to basic disk usage with df
    println!("{} iostat not found, showing disk usage instead:", "ℹ️".yellow());
    println!();

    let output = Command::new("df")
        .args(&["-h"])
        .output()
        .map_err(|e| format!("Failed to run df: {}", e))?;

    if !output.status.success() {
        return Err("df command failed".to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for (i, line) in stdout.lines().enumerate() {
        if i == 0 {
            println!("{}", line.bold());
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}

fn parse_iostat(output: &str) {
    let mut in_device_section = false;
    let mut header_printed = false;

    for line in output.lines() {
        if line.contains("Device") && line.contains("r/s") {
            in_device_section = true;
            if !header_printed {
                println!("  {:<12} {:<8} {:<8} {:<10} {:<10}", 
                    "Device".bold(), 
                    "r/s".bold(), 
                    "w/s".bold(), 
                    "rkB/s".bold(), 
                    "wkB/s".bold()
                );
                println!("  {}", "─".repeat(60).dimmed());
                header_printed = true;
            }
            continue;
        }

        if in_device_section && !line.trim().is_empty() && !line.starts_with("avg") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let device = parts[0];
                let r_s = parts[3];
                let w_s = parts[4];
                let rkb_s = parts[5];
                let wkb_s = parts[6];

                println!("  {:<12} {:<8} {:<8} {:<10} {:<10}",
                    device.cyan(),
                    r_s.yellow(),
                    w_s.yellow(),
                    rkb_s.green(),
                    wkb_s.green()
                );
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
