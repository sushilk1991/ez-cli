use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    file: PathBuf,
    numbers: bool,
    first: Option<usize>,
    last: Option<usize>,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot read '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot read '{}': {}", file.display(), e))
        }
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

    if !ctx.json {
        let width = lines_to_show.len().to_string().len();

        for (i, line) in lines_to_show.iter().enumerate() {
            if numbers {
                let line_num = if first.is_some() {
                    i + 1
                } else if last.is_some() {
                    total_lines - lines_to_show.len() + i + 1
                } else {
                    i + 1
                };
                println!("{:>width$} │ {}", line_num.to_string().dimmed(), line, width = width);
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(CommandOutput::new("show", serde_json::json!({
        "file": file.display().to_string(),
        "total_lines": total_lines,
        "lines_shown": lines_to_show.len(),
        "content": lines_to_show.join("\n"),
    })))
}
