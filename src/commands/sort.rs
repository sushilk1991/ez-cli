use std::fs;
use std::path::PathBuf;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(file: PathBuf, reverse: bool, numeric: bool, unique: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            EzError::NotFound(format!("Cannot read '{}': {}", file.display(), e))
        } else {
            EzError::General(format!("Cannot read '{}': {}", file.display(), e))
        }
    })?;

    let mut lines: Vec<String> = contents.lines().map(String::from).collect();

    if numeric {
        lines.sort_by(|a, b| {
            let a_num = a.parse::<f64>().unwrap_or(f64::INFINITY);
            let b_num = b.parse::<f64>().unwrap_or(f64::INFINITY);
            a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        lines.sort();
    }

    if unique {
        lines.dedup();
    }

    if reverse {
        lines.reverse();
    }

    if !ctx.json {
        for line in &lines {
            println!("{}", line);
        }
    }

    Ok(CommandOutput::new("sort", serde_json::json!({
        "file": file.display().to_string(),
        "lines": lines,
    })))
}
