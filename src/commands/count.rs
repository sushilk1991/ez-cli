use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(files: Vec<PathBuf>, lines: bool, words: bool, bytes: bool) -> Result<(), String> {
    let mut total_lines = 0usize;
    let mut total_words = 0usize;
    let mut total_bytes = 0usize;
    let mut file_count = 0;

    for file in &files {
        let contents = fs::read_to_string(file).map_err(|e| {
            format!("Cannot read '{}': {}", file.display(), e)
        })?;

        let line_count = contents.lines().count();
        let word_count = contents.split_whitespace().count();
        let byte_count = contents.len();

        total_lines += line_count;
        total_words += word_count;
        total_bytes += byte_count;
        file_count += 1;

        print_counts(file.display().to_string(), line_count, word_count, byte_count, lines, words, bytes);
    }

    if file_count > 1 {
        print_counts("total".to_string().green().bold().to_string(), total_lines, total_words, total_bytes, lines, words, bytes);
    }

    Ok(())
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
