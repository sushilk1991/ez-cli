use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(
    file: PathBuf,
    numbers: bool,
    first: Option<usize>,
    last: Option<usize>,
) -> Result<(), String> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        format!("Cannot read '{}': {}", file.display(), e)
    })?;

    let lines: Vec<_> = contents.lines().collect();
    let total_lines = lines.len();

    let lines_to_show: Vec<_> = if let Some(n) = first {
        lines.into_iter().take(n).collect()
    } else if let Some(n) = last {
        lines.into_iter().skip(total_lines.saturating_sub(n)).collect()
    } else {
        lines
    };

    let width = lines_to_show.len().to_string().len();

    for (i, line) in lines_to_show.iter().enumerate() {
        if numbers {
            let line_num = if let Some(_n) = first {
                i + 1
            } else if let Some(_) = last {
                total_lines - lines_to_show.len() + i + 1
            } else {
                i + 1
            };
            println!("{:>width$} │ {}", line_num.to_string().dimmed(), line, width = width);
        } else {
            println!("{}", line);
        }
    }

    Ok(())
}
