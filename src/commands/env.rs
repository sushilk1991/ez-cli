use std::env;
use colored::*;

pub fn execute(filter: Option<String>) -> Result<(), String> {
    let vars: Vec<(String, String)> = env::vars().collect();

    let filtered: Vec<_> = if let Some(pattern) = filter {
        vars.into_iter()
            .filter(|(key, _)| key.contains(&pattern) || key.to_lowercase().contains(&pattern.to_lowercase()))
            .collect()
    } else {
        vars
    };

    if filtered.is_empty() {
        println!("{} No environment variables found", "ℹ️".yellow());
        return Ok(());
    }

    println!("{} Found {} environment variables", "🌍".green(), filtered.len());
    println!();

    let mut sorted = filtered;
    sorted.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in sorted {
        println!("{} = {}", key.cyan().bold(), value.dimmed());
    }

    Ok(())
}
