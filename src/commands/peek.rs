use std::fs;
use std::path::PathBuf;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    file: PathBuf,
    lines: usize,
    tail: bool,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot read '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot read '{}': {}", file.display(), e))
        }
    })?;

    let all_lines: Vec<_> = contents.lines().collect();
    let total = all_lines.len();

    let lines_to_show: Vec<_> = if tail {
        all_lines.into_iter().skip(total.saturating_sub(lines)).collect()
    } else {
        all_lines.into_iter().take(lines).collect()
    };

    if !ctx.json {
        println!("{} Showing {} {} lines of {}",
            if tail { "📜" } else { "📄" },
            lines_to_show.len(),
            if tail { "last" } else { "first" },
            file.display().to_string().cyan()
        );
        println!();

        for line in &lines_to_show {
            println!("{}", line);
        }
    }

    Ok(CommandOutput::new("peek", serde_json::json!({
        "file": file.display().to_string(),
        "position": if tail { "tail" } else { "head" },
        "total_lines": total,
        "lines_shown": lines_to_show.len(),
        "content": lines_to_show.join("\n"),
    })))
}
