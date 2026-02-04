use std::env;
use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(filter: Option<String>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let vars: Vec<(String, String)> = env::vars().collect();

    let filtered: Vec<_> = if let Some(pattern) = filter {
        vars.into_iter()
            .filter(|(key, _)| key.contains(&pattern) || key.to_lowercase().contains(&pattern.to_lowercase()))
            .collect()
    } else {
        vars
    };

    let mut sorted = filtered;
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    if !ctx.json {
        if sorted.is_empty() {
            println!("{} No environment variables found", "ℹ️".yellow());
            return Ok(CommandOutput::new("env", serde_json::json!([])));
        }

        println!("{} Found {} environment variables", "🌍".green(), sorted.len());
        println!();

        for (key, value) in &sorted {
            println!("{} = {}", key.cyan().bold(), value.dimmed());
        }
    }

    let json_vars: Vec<_> = sorted.iter().map(|(k, v)| serde_json::json!({ "name": k, "value": v })).collect();
    Ok(CommandOutput::new("env", serde_json::json!(json_vars)))
}
