use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    pattern: String,
    path: PathBuf,
    context: usize,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let mut total_matches = 0;
    let mut files_with_matches = 0;
    let mut results = Vec::new();

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let file_path = entry.path();
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let lines: Vec<&str> = contents.lines().collect();
        let mut matches: Vec<usize> = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            if line.contains(&pattern) {
                matches.push(idx);
                total_matches += 1;
            }
        }

        if !matches.is_empty() {
            files_with_matches += 1;
            let file_str = file_path.display().to_string();

            let json_matches: Vec<_> = matches.iter().map(|&idx| {
                serde_json::json!({
                    "line": idx + 1,
                    "text": lines[idx],
                })
            }).collect();

            results.push(serde_json::json!({
                "file": file_str,
                "matches": json_matches,
            }));

            if !ctx.json {
                println!("\n{} {}", "📁".cyan(), file_str.bold());

                for &match_idx in &matches {
                    let start = match_idx.saturating_sub(context);
                    for i in start..match_idx {
                        println!("  {} {}", (i + 1).to_string().dimmed(), lines[i].dimmed());
                    }

                    let highlighted = lines[match_idx].replace(&pattern, &pattern.yellow().to_string());
                    println!("  {} {}", (match_idx + 1).to_string().green().bold(), highlighted);

                    let end = (match_idx + context + 1).min(lines.len());
                    for i in (match_idx + 1)..end {
                        println!("  {} {}", (i + 1).to_string().dimmed(), lines[i].dimmed());
                    }
                }
            }
        }
    }

    if !ctx.json {
        println!("\n{} Found {} matches in {} files", "✓".green(), total_matches, files_with_matches);
    }

    Ok(CommandOutput::new("search", serde_json::json!(results))
        .with_metadata(serde_json::json!({
            "total_matches": total_matches,
            "total_files": files_with_matches,
        })))
}
