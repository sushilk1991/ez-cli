use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use regex::Regex;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    pattern: String,
    path: PathBuf,
    inside: bool,
    ignore_case: bool,
    line_numbers: bool,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    if inside {
        find_in_contents(pattern, path, ignore_case, line_numbers, ctx)
    } else {
        find_files(pattern, path, ctx)
    }
}

fn find_files(pattern: String, path: PathBuf, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let re = Regex::new(&format!("(?i){}", regex::escape(&pattern)))
        .map_err(|e| EzError::InvalidArgs(format!("Invalid pattern: {}", e)))?;

    let mut found = 0;
    let mut results = Vec::new();

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy();

        if re.is_match(&name) {
            let path_display = entry.path().strip_prefix(&path).unwrap_or(entry.path());
            let path_str = path_display.display().to_string();
            results.push(serde_json::json!({
                "path": path_str,
                "type": if entry.file_type().is_dir() { "directory" } else { "file" },
            }));

            if !ctx.json {
                println!("{}", path_str.green());
            }
            found += 1;
        }
    }

    if !ctx.json {
        if found == 0 {
            println!("{} No files matching '{}' found", "ℹ".blue(), pattern);
        } else {
            println!("\n{} Found {} file(s)", "✓".green(), found);
        }
    }

    Ok(CommandOutput::new("find", serde_json::json!(results)))
}

fn find_in_contents(
    pattern: String,
    path: PathBuf,
    ignore_case: bool,
    line_numbers: bool,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let re = if ignore_case {
        Regex::new(&format!("(?i){}", regex::escape(&pattern)))
    } else {
        Regex::new(&regex::escape(&pattern))
    }.map_err(|e| EzError::InvalidArgs(format!("Invalid pattern: {}", e)))?;

    let mut found_files = 0;
    let mut found_matches = 0;
    let mut results = Vec::new();

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(contents) = fs::read_to_string(entry.path()) {
            let mut file_matches = Vec::new();

            for (line_num, line) in contents.lines().enumerate() {
                if re.is_match(line) {
                    file_matches.push(serde_json::json!({
                        "line": line_num + 1,
                        "text": line,
                    }));
                    found_matches += 1;
                }
            }

            if !file_matches.is_empty() {
                found_files += 1;
                let path_display = entry.path().strip_prefix(&path).unwrap_or(entry.path());
                let path_str = path_display.display().to_string();

                results.push(serde_json::json!({
                    "file": path_str,
                    "matches": file_matches,
                }));

                if !ctx.json {
                    println!("\n{}", path_str.cyan().underline());
                    for m in &file_matches {
                        let ln = m["line"].as_u64().unwrap_or(0);
                        let text = m["text"].as_str().unwrap_or("");
                        let highlighted = re.replace_all(text, |caps: &regex::Captures| {
                            caps[0].to_string().red().bold().to_string()
                        });
                        if line_numbers {
                            println!("  {:>4} │ {}", ln.to_string().dimmed(), highlighted);
                        } else {
                            println!("  {}", highlighted);
                        }
                    }
                }
            }
        }
    }

    if !ctx.json {
        if found_matches == 0 {
            println!("{} No matches for '{}' found", "ℹ".blue(), pattern);
        } else {
            println!("\n{} Found {} match(es) in {} file(s)",
                "✓".green(), found_matches, found_files);
        }
    }

    Ok(CommandOutput::new("find", serde_json::json!(results))
        .with_metadata(serde_json::json!({
            "total_matches": found_matches,
            "total_files": found_files,
            "mode": "content",
        })))
}
