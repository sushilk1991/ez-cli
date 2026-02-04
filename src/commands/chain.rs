use colored::*;
use serde::{Deserialize, Serialize};
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

#[derive(Serialize, Deserialize)]
pub struct ChainResult {
    pub input: String,
    pub pipeline: String,
    pub steps: Vec<ChainStep>,
}

#[derive(Serialize, Deserialize)]
pub struct ChainStep {
    pub step: usize,
    pub command: String,
    pub explanation: String,
}

pub fn run(query: &str, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let pipeline = build_pipeline(query);

    if ctx.json {
        let result = ChainResult {
            input: query.to_string(),
            pipeline: pipeline.pipeline.clone(),
            steps: pipeline.steps,
        };
        let data = serde_json::to_value(&result).unwrap_or(serde_json::json!({}));
        return Ok(CommandOutput::new("chain", data));
    }

    println!("{}", "🔗 Pipeline:".bold());
    println!("  {}", pipeline.pipeline.green().bold());
    println!();
    println!("{}", "📖 Explanation:".bold());
    for step in &pipeline.steps {
        println!("  Step {}: {} → {}", step.step, step.command.cyan(), step.explanation);
    }
    println!();
    println!("{} {}", "💡 Copy and run:".dimmed(), pipeline.pipeline);

    Ok(CommandOutput::new("chain", serde_json::json!({
        "input": query,
        "pipeline": pipeline.pipeline,
    })))
}

struct PipelineResult {
    pipeline: String,
    steps: Vec<ChainStep>,
}

fn build_pipeline(query: &str) -> PipelineResult {
    let q = query.to_lowercase();
    let mut commands: Vec<(String, String)> = Vec::new();
    
    // Pattern matching on common queries
    
    // File finding patterns
    if q.contains("find") || q.contains("search") || q.contains("look for") {
        let mut find_cmd = "find .".to_string();
        
        // File type detection
        let extensions = ["py", "rs", "js", "ts", "go", "java", "rb", "sh", "txt", "md", "json", "yaml", "yml", "toml", "csv", "html", "css"];
        for ext in extensions {
            if q.contains(&format!(".{}", ext)) || q.contains(&format!("{} file", ext)) || q.contains(&format!("{} files", ext)) {
                find_cmd.push_str(&format!(" -name \"*.{}\"", ext));
                break;
            }
        }
        
        // Size filters
        if q.contains("large") || q.contains("big") || q.contains("over") {
            if let Some(size) = extract_size(&q) {
                find_cmd.push_str(&format!(" -size +{}", size));
            } else {
                find_cmd.push_str(" -size +10M");
            }
        } else if q.contains("small") || q.contains("tiny") {
            find_cmd.push_str(" -size -1M");
        }
        
        // Type filters
        if q.contains("director") || q.contains("folder") {
            find_cmd.push_str(" -type d");
        } else if !find_cmd.contains("-name") {
            find_cmd.push_str(" -type f");
        }
        
        commands.push((find_cmd, "Find matching files".to_string()));
    }
    
    // Counting patterns
    if q.contains("count") || q.contains("how many") {
        if q.contains("line") {
            if commands.is_empty() {
                commands.push(("find . -type f".to_string(), "Find all files".to_string()));
            }
            commands.push(("xargs wc -l".to_string(), "Count lines in each file".to_string()));
        } else if commands.is_empty() {
            commands.push(("find . -type f".to_string(), "Find all files".to_string()));
            commands.push(("wc -l".to_string(), "Count total files".to_string()));
        }
    }
    
    // Sorting patterns
    if q.contains("sort") || q.contains("order") || q.contains("biggest") || q.contains("largest") || q.contains("top") {
        if q.contains("smallest") || q.contains("ascending") {
            commands.push(("sort -n".to_string(), "Sort numerically (ascending)".to_string()));
        } else {
            commands.push(("sort -rn".to_string(), "Sort numerically (largest first)".to_string()));
        }
    }
    
    // Top/head patterns
    if q.contains("top") || q.contains("first") {
        let n = extract_number(&q).unwrap_or(10);
        commands.push((format!("head -{}", n), format!("Show top {} results", n)));
    } else if q.contains("last") || q.contains("bottom") {
        let n = extract_number(&q).unwrap_or(10);
        commands.push((format!("tail -{}", n), format!("Show last {} results", n)));
    }
    
    // Grep/filter patterns
    if q.contains("contain") || q.contains("with text") || q.contains("matching") || q.contains("grep") {
        if let Some(pattern) = extract_quoted(query) {
            commands.push((format!("grep \"{}\"", pattern), format!("Filter lines containing '{}'", pattern)));
        } else {
            commands.push(("grep \"PATTERN\"".to_string(), "Filter matching lines".to_string()));
        }
    }
    
    // Unique/deduplicate
    if q.contains("unique") || q.contains("dedup") || q.contains("duplicate") {
        if q.contains("duplicate") {
            commands.push(("sort".to_string(), "Sort for grouping".to_string()));
            commands.push(("uniq -d".to_string(), "Show only duplicates".to_string()));
        } else {
            commands.push(("sort -u".to_string(), "Sort and remove duplicates".to_string()));
        }
    }
    
    // Replace patterns
    if q.contains("replace") || q.contains("change") || q.contains("substitute") {
        commands.push(("sed 's/OLD/NEW/g'".to_string(), "Replace OLD with NEW".to_string()));
    }
    
    // Disk usage
    if q.contains("disk") || q.contains("space") || q.contains("usage") {
        if commands.is_empty() {
            commands.push(("du -sh *".to_string(), "Show size of each item".to_string()));
            commands.push(("sort -rh".to_string(), "Sort by size (largest first)".to_string()));
        }
    }
    
    // Fallback
    if commands.is_empty() {
        commands.push(("echo \"Could not parse query. Try being more specific.\"".to_string(), "No matching pattern found".to_string()));
    }
    
    let pipeline = commands.iter().map(|(c, _)| c.as_str()).collect::<Vec<_>>().join(" | ");
    let steps = commands.iter().enumerate().map(|(i, (cmd, desc))| {
        ChainStep { step: i + 1, command: cmd.clone(), explanation: desc.clone() }
    }).collect();
    
    PipelineResult { pipeline, steps }
}

fn extract_size(q: &str) -> Option<String> {
    // Look for patterns like "100MB", "1GB", "50kb"
    let re_patterns = [
        ("gb", "G"), ("mb", "M"), ("kb", "K"),
        ("g", "G"), ("m", "M"), ("k", "K"),
    ];
    for word in q.split_whitespace() {
        for (suffix, unit) in &re_patterns {
            if word.ends_with(suffix) {
                if let Ok(num) = word[..word.len()-suffix.len()].parse::<u64>() {
                    return Some(format!("{}{}", num, unit));
                }
            }
        }
    }
    None
}

fn extract_number(q: &str) -> Option<usize> {
    for word in q.split_whitespace() {
        if let Ok(n) = word.parse::<usize>() {
            return Some(n);
        }
    }
    None
}

fn extract_quoted(q: &str) -> Option<String> {
    if let Some(start) = q.find('"') {
        if let Some(end) = q[start+1..].find('"') {
            return Some(q[start+1..start+1+end].to_string());
        }
    }
    if let Some(start) = q.find('\'') {
        if let Some(end) = q[start+1..].find('\'') {
            return Some(q[start+1..start+1+end].to_string());
        }
    }
    None
}
