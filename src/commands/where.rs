use std::env;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let current = env::current_dir().map_err(|e| EzError::General(format!("Cannot get current directory: {}", e)))?;
    let path_str = current.display().to_string();

    if !ctx.json {
        println!("{}", path_str.cyan());
    }

    Ok(CommandOutput::new("where", serde_json::json!({ "path": path_str })))
}
