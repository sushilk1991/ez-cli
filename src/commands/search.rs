use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;

pub fn execute(
    pattern: String,
    path: PathBuf,
    context: usize,
) -> Result<(), String> {
    let mut total_matches = 0;
    let mut files_with_matches = 0;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let file_path = entry.path();
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(_) => continue, // Skip binary or unreadable files
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
            println!("\n{} {}", "📁".cyan(), file_path.display().to_string().bold());

            for &match_idx in &matches {
                // Show context lines before
                let start = match_idx.saturating_sub(context);
                for i in start..match_idx {
                    println!("  {} {}", (i + 1).to_string().dimmed(), lines[i].dimmed());
                }

                // Show matching line
                let highlighted = lines[match_idx].replace(&pattern, &pattern.yellow().to_string());
                println!("  {} {}", (match_idx + 1).to_string().green().bold(), highlighted);

                // Show context lines after
                let end = (match_idx + context + 1).min(lines.len());
                for i in (match_idx + 1)..end {
                    println!("  {} {}", (i + 1).to_string().dimmed(), lines[i].dimmed());
                }
            }
        }
    }

    println!("\n{} Found {} matches in {} files", "✓".green(), total_matches, files_with_matches);
    Ok(())
}
