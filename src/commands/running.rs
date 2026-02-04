use std::process::Command;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(all: bool, filter: Option<String>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let output = if all {
        Command::new("ps").args(&["aux"]).output()
    } else {
        Command::new("ps").args(&["-eo", "pid,ppid,%cpu,%mem,comm,etime"]).output()
    }.map_err(|e| EzError::General(format!("Failed to run ps: {}", e)))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    if lines.is_empty() {
        if !ctx.json {
            println!("{} No running processes found", "ℹ".blue());
        }
        return Ok(CommandOutput::new("running", serde_json::json!([])));
    }

    let header = lines[0];
    let processes: Vec<&str> = if let Some(ref pattern) = filter {
        lines[1..].iter()
            .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
            .copied()
            .collect()
    } else {
        lines[1..].to_vec()
    };

    if !ctx.json {
        if processes.is_empty() && filter.is_some() {
            println!("{} No processes matching '{}' found", "ℹ".blue(), filter.unwrap());
            return Ok(CommandOutput::new("running", serde_json::json!([])));
        }

        println!("{}", header.dimmed());
        for line in &processes {
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
    }

    let json_procs: Vec<_> = processes.iter().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        serde_json::json!({
            "raw": line,
            "pid": parts.first().unwrap_or(&""),
        })
    }).collect();

    Ok(CommandOutput::new("running", serde_json::json!(json_procs)))
}
