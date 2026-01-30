use std::process::Command;
use colored::*;

pub fn execute(all: bool, filter: Option<String>) -> Result<(), String> {
    let output = if all {
        Command::new("ps")
            .args(&["aux"])
            .output()
            .map_err(|e| format!("Failed to run ps: {}", e))?
    } else {
        Command::new("ps")
            .args(&["-eo", "pid,ppid,%cpu,%mem,comm,etime"])
            .output()
            .map_err(|e| format!("Failed to run ps: {}", e))?
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        println!("{} No running processes found", "ℹ".blue());
        return Ok(());
    }

    println!("{}", lines[0].dimmed());

    let processes: Vec<&str> = if let Some(ref pattern) = filter {
        lines[1..].iter()
            .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
            .copied()
            .collect()
    } else {
        lines[1..].to_vec()
    };

    if processes.is_empty() && filter.is_some() {
        println!("{} No processes matching '{}' found", "ℹ".blue(), filter.unwrap());
        return Ok(());
    }

    for line in processes {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let pid = parts[0];
            let pid_colored = pid.yellow();
            let rest = &line[pid.len()..];
            println!("{}{}", pid_colored, rest);
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
