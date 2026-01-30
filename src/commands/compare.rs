use std::fs;
use std::path::PathBuf;
use colored::*;
use similar::{ChangeTag, TextDiff};

pub fn execute(file1: PathBuf, file2: PathBuf, side_by_side: bool) -> Result<(), String> {
    let contents1 = fs::read_to_string(&file1).map_err(|e| {
        format!("Cannot read '{}': {}", file1.display(), e)
    })?;

    let contents2 = fs::read_to_string(&file2).map_err(|e| {
        format!("Cannot read '{}': {}", file2.display(), e)
    })?;

    if side_by_side {
        show_side_by_side(&file1, &file2, &contents1, &contents2)?;
    } else {
        show_unified(&file1, &file2, &contents1, &contents2);
    }

    Ok(())
}

fn show_unified(file1: &PathBuf, file2: &PathBuf, contents1: &str, contents2: &str) {
    let diff = TextDiff::from_lines(contents1, contents2);
    
    println!("{} {}", "---".red(), file1.display());
    println!("{} {}", "+++".green(), file2.display());

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-".red(),
            ChangeTag::Insert => "+".green(),
            ChangeTag::Equal => " ".normal(),
        };
        print!("{}{}", sign, change);
    }
}

fn show_side_by_side(file1: &PathBuf, file2: &PathBuf, contents1: &str, contents2: &str) -> Result<(), String> {
    let lines1: Vec<&str> = contents1.lines().collect();
    let lines2: Vec<&str> = contents2.lines().collect();

    let max_len = lines1.len().max(lines2.len());
    let width = 40;

    println!("{:<width$} │ {}", 
        file1.display().to_string().red().underline(),
        file2.display().to_string().green().underline(),
        width = width);
    println!("{}", "─".repeat(width * 2 + 3));

    for i in 0..max_len {
        let line1 = lines1.get(i).unwrap_or(&"");
        let line2 = lines2.get(i).unwrap_or(&"");

        let (colored1, colored2) = if line1 != line2 {
            (line1.red().to_string(), line2.green().to_string())
        } else {
            (line1.normal().to_string(), line2.normal().to_string())
        };

        let truncated1 = if colored1.len() > width {
            format!("{}...", &colored1[..width-3])
        } else {
            colored1
        };

        let truncated2 = if colored2.len() > width {
            format!("{}...", &colored2[..width-3])
        } else {
            colored2
        };

        println!("{:<width$} │ {}", truncated1, truncated2, width = width);
    }

    Ok(())
}
