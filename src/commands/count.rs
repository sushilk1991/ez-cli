use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(files: Vec<PathBuf>, lines: bool, words: bool, bytes: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let mut total_lines = 0usize;
    let mut total_words = 0usize;
    let mut total_bytes = 0usize;
    let mut results = Vec::new();

    for file in &files {
        let contents = fs::read_to_string(file).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                EzError::NotFound(format!("Cannot read '{}': {}", file.display(), e))
            } else {
                EzError::General(format!("Cannot read '{}': {}", file.display(), e))
            }
        })?;

        let line_count = contents.lines().count();
        let word_count = contents.split_whitespace().count();
        let byte_count = contents.len();

        total_lines += line_count;
        total_words += word_count;
        total_bytes += byte_count;

        results.push(serde_json::json!({
            "file": file.display().to_string(),
            "lines": line_count,
            "words": word_count,
            "bytes": byte_count,
        }));

        if !ctx.json {
            print_counts(file.display().to_string(), line_count, word_count, byte_count, lines, words, bytes);
        }
    }

    if !ctx.json && files.len() > 1 {
        print_counts("total".to_string().green().bold().to_string(), total_lines, total_words, total_bytes, lines, words, bytes);
    }

    Ok(CommandOutput::new("count", serde_json::json!({
        "files": results,
        "total": { "lines": total_lines, "words": total_words, "bytes": total_bytes },
    })))
}

fn print_counts(name: String, lines: usize, words: usize, bytes: usize, lines_only: bool, words_only: bool, bytes_only: bool) {
    if lines_only {
        println!("{:>8} {}", lines.to_string().cyan(), name);
    } else if words_only {
        println!("{:>8} {}", words.to_string().cyan(), name);
    } else if bytes_only {
        println!("{:>8} {}", bytes.to_string().cyan(), name);
    } else {
        println!("{:>8} {:>8} {:>8} {}",
            lines.to_string().cyan(),
            words.to_string().yellow(),
            bytes.to_string().green(),
            name);
    }
}
