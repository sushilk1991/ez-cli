use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(
    file: PathBuf,
    lines: usize,
    tail: bool,
) -> Result<(), String> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        format!("Cannot read '{}': {}", file.display(), e)
    })?;

    let all_lines: Vec<_> = contents.lines().collect();
    let total = all_lines.len();

    let lines_to_show: Vec<_> = if tail {
        all_lines.into_iter().skip(total.saturating_sub(lines)).collect()
    } else {
        all_lines.into_iter().take(lines).collect()
    };

    println!("{} Showing {} {} lines of {}", 
        if tail { "📜" } else { "📄" },
        lines_to_show.len(),
        if tail { "last" } else { "first" },
        file.display().to_string().cyan()
    );
    println!();

    for line in lines_to_show {
        println!("{}", line);
    }

    Ok(())
}
