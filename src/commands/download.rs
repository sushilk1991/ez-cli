use std::fs::File;
use std::io::{Write, Read};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(url: String, save: Option<String>, progress: bool, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let filename = save.unwrap_or_else(|| {
        url.split('/').last().unwrap_or("download").to_string()
    });

    if !ctx.json {
        println!("{} Downloading from {}", "⬇️".cyan(), url.dimmed());
    }

    let response = ureq::get(&url)
        .call()
        .map_err(|e| EzError::General(format!("Download failed: {}", e)))?;

    let total_size = response.header("Content-Length")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    let mut file = File::create(&filename)
        .map_err(|e| EzError::General(format!("Cannot create file: {}", e)))?;

    let mut reader = response.into_reader();

    if progress && !ctx.json && total_size > 0 {
        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut buffer = [0; 8192];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => {
                    file.write_all(&buffer[..n]).map_err(|e| EzError::General(format!("Write failed: {}", e)))?;
                    pb.inc(n as u64);
                }
                Err(e) => return Err(EzError::General(format!("Read error: {}", e))),
            }
        }
        pb.finish_and_clear();
    } else {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).map_err(|e| EzError::General(format!("Read failed: {}", e)))?;
        file.write_all(&buffer).map_err(|e| EzError::General(format!("Write failed: {}", e)))?;
    }

    let size = file.metadata().map(|m| m.len()).unwrap_or(0);

    if !ctx.json {
        println!("{} Saved to {} ({})", "✓".green(), filename.cyan(), crate::utils::format_size(size));
    }

    Ok(CommandOutput::new("download", serde_json::json!({
        "url": url,
        "file": filename,
        "size": size,
    })))
}
