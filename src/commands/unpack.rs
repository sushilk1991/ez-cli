use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use colored::*;
use flate2::read::GzDecoder;
use bzip2::read::BzDecoder;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(archive: PathBuf, to: Option<PathBuf>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let destination = to.unwrap_or_else(|| PathBuf::from("."));

    if !destination.exists() {
        std::fs::create_dir_all(&destination)
            .map_err(|e| EzError::General(format!("Cannot create destination: {}", e)))?;
    }

    let ext = archive.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase());

    let result = match ext.as_deref() {
        Some("zip") => unpack_zip(&archive, &destination),
        Some("tar") => unpack_tar(&archive, &destination, None),
        Some("gz") | Some("tgz") => unpack_tar(&archive, &destination, Some(Compression::Gzip)),
        Some("bz2") => unpack_tar(&archive, &destination, Some(Compression::Bzip2)),
        _ => Err(EzError::InvalidArgs("Unknown archive format. Supported: .zip, .tar, .tar.gz, .tgz, .tar.bz2".to_string())),
    }?;

    if !ctx.json {
        println!("{} Extracted {} to {}", "✓".green(), archive.display().to_string().cyan(), destination.display());
    }

    Ok(result)
}

#[derive(Clone, Copy)]
enum Compression { Gzip, Bzip2 }

fn unpack_zip(archive: &PathBuf, destination: &PathBuf) -> Result<CommandOutput, EzError> {
    let file = File::open(archive).map_err(|e| EzError::NotFound(format!("Cannot open archive: {}", e)))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| EzError::General(format!("Invalid zip: {}", e)))?;

    let mut extracted = Vec::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| EzError::General(format!("Zip error: {}", e)))?;
        let entry_path = destination.join(entry.name());
        extracted.push(entry.name().to_string());

        if entry.is_dir() {
            std::fs::create_dir_all(&entry_path).map_err(|e| EzError::General(format!("Create dir error: {}", e)))?;
        } else {
            if let Some(parent) = entry_path.parent() {
                std::fs::create_dir_all(parent).map_err(|e| EzError::General(format!("Create dir error: {}", e)))?;
            }
            let mut outfile = File::create(&entry_path).map_err(|e| EzError::General(format!("Create file error: {}", e)))?;
            let mut buffer = vec![0; entry.size() as usize];
            entry.read_exact(&mut buffer).map_err(|e| EzError::General(format!("Read error: {}", e)))?;
            outfile.write_all(&buffer).map_err(|e| EzError::General(format!("Write error: {}", e)))?;
        }
    }

    Ok(CommandOutput::new("unpack", serde_json::json!({
        "archive": archive.display().to_string(),
        "destination": destination.display().to_string(),
        "files": extracted,
    })))
}

fn unpack_tar(archive: &PathBuf, destination: &PathBuf, compression: Option<Compression>) -> Result<CommandOutput, EzError> {
    let file = File::open(archive).map_err(|e| EzError::NotFound(format!("Cannot open archive: {}", e)))?;

    let tar: Box<dyn Read> = match compression {
        Some(Compression::Gzip) => Box::new(GzDecoder::new(file)),
        Some(Compression::Bzip2) => Box::new(BzDecoder::new(file)),
        None => Box::new(file),
    };

    let mut tar = tar::Archive::new(tar);
    tar.unpack(destination).map_err(|e| EzError::General(format!("Tar extract error: {}", e)))?;

    Ok(CommandOutput::new("unpack", serde_json::json!({
        "archive": archive.display().to_string(),
        "destination": destination.display().to_string(),
    })))
}
