use std::process::Command;
use colored::*;

pub fn execute(target: String, force: bool) -> Result<(), String> {
    // Try to parse as PID first
    if let Ok(pid) = target.parse::<i32>() {
        stop_by_pid(pid, force)?;
    } else {
        stop_by_name(&target, force)?;
    }
    Ok(())
}

fn stop_by_pid(pid: i32, force: bool) -> Result<(), String> {
    let signal = if force { "-9" } else { "-15" };
    
    let output = Command::new("kill")
        .args(&[signal, &pid.to_string()])
        .output()
        .map_err(|e| format!("Failed to execute kill: {}", e))?;

    if output.status.success() {
        println!("{} Stopped process {}", "✓".green(), pid.to_string().yellow());
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to stop process {}: {}", pid, stderr))
    }
}

fn stop_by_name(name: &str, force: bool) -> Result<(), String> {
    let signal = if force { "-9" } else { "-15" };
    
    let output = Command::new("pkill")
        .args(&[signal, name])
        .output()
        .map_err(|e| format!("Failed to execute pkill: {}", e))?;

    if output.status.success() {
        println!("{} Stopped process(es) matching '{}'", "✓".green(), name.yellow());
        Ok(())
    } else {
        Err(format!("No process found matching '{}'", name))
    }
}
