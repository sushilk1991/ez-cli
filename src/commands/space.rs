use std::process::Command;
use colored::*;

pub fn execute() -> Result<(), String> {
    let output = Command::new("df")
        .args(&["-h"])
        .output()
        .map_err(|e| format!("Failed to run df: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        return Err("No disk information available".to_string());
    }

    println!("{}", lines[0].bold().underline());

    for line in &lines[1..] {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 6 {
            // Check if it's a real filesystem (not tmpfs, devtmpfs, etc.)
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

    Ok(())
}
