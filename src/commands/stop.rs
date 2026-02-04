use std::process::Command;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(target: String, force: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    if let Ok(pid) = target.parse::<i32>() {
        stop_by_pid(pid, force, ctx)
    } else {
        stop_by_name(&target, force, ctx)
    }
}

fn stop_by_pid(pid: i32, force: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let signal = if force { "-9" } else { "-15" };

    if ctx.dry_run {
        if !ctx.json {
            println!("{} Would send {} to process {}", "~".dimmed(), signal, pid);
        }
        return Ok(CommandOutput::new("stop", serde_json::json!({
            "target": pid.to_string(),
            "signal": signal,
        })).with_metadata(serde_json::json!({ "dry_run": true })));
    }

    if ctx.should_confirm() {
        print!("Stop process {}? [y/N] ", pid);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            return Err(EzError::Cancelled("Stop cancelled by user".to_string()));
        }
    }

    let output = Command::new("kill")
        .args(&[signal, &pid.to_string()])
        .output()
        .map_err(|e| EzError::General(format!("Failed to execute kill: {}", e)))?;

    if output.status.success() {
        if !ctx.json {
            println!("{} Stopped process {}", "✓".green(), pid.to_string().yellow());
        }
        Ok(CommandOutput::new("stop", serde_json::json!({
            "target": pid.to_string(),
            "signal": signal,
        })))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(EzError::General(format!("Failed to stop process {}: {}", pid, stderr)))
    }
}

fn stop_by_name(name: &str, force: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let signal = if force { "-9" } else { "-15" };

    if ctx.dry_run {
        if !ctx.json {
            println!("{} Would send {} to processes matching '{}'", "~".dimmed(), signal, name);
        }
        return Ok(CommandOutput::new("stop", serde_json::json!({
            "target": name,
            "signal": signal,
        })).with_metadata(serde_json::json!({ "dry_run": true })));
    }

    if ctx.should_confirm() {
        print!("Stop process(es) matching '{}'? [y/N] ", name);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if !input.trim().eq_ignore_ascii_case("y") {
            return Err(EzError::Cancelled("Stop cancelled by user".to_string()));
        }
    }

    let output = Command::new("pkill")
        .args(&[signal, name])
        .output()
        .map_err(|e| EzError::General(format!("Failed to execute pkill: {}", e)))?;

    if output.status.success() {
        if !ctx.json {
            println!("{} Stopped process(es) matching '{}'", "✓".green(), name.yellow());
        }
        Ok(CommandOutput::new("stop", serde_json::json!({
            "target": name,
            "signal": signal,
        })))
    } else {
        Err(EzError::NotFound(format!("No process found matching '{}'", name)))
    }
}
