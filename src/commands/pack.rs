use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use zip::write::FileOptions;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(
    archive: PathBuf,
    files: Vec<PathBuf>,
    format: Option<crate::ArchiveFormat>,
    ctx: &CommandContext,
) -> Result<CommandOutput, EzError> {
    let format = format.or_else(|| detect_format(&archive));

    match format {
        Some(crate::ArchiveFormat::Zip) => pack_zip(archive, files, ctx),
        Some(crate::ArchiveFormat::Tar) => pack_tar(archive, files, None, ctx),
        Some(crate::ArchiveFormat::TarGz) => pack_tar(archive, files, Some(flate2::Compression::default()), ctx),
        Some(crate::ArchiveFormat::TarBz2) => pack_tar_bz2(archive, files, ctx),
        None => Err(EzError::InvalidArgs("Cannot detect archive format from extension. Use --format".to_string())),
    }
}

fn detect_format(path: &PathBuf) -> Option<crate::ArchiveFormat> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    match ext.as_str() {
        "zip" => Some(crate::ArchiveFormat::Zip),
        "tar" => Some(crate::ArchiveFormat::Tar),
        "gz" | "tgz" => Some(crate::ArchiveFormat::TarGz),
        "bz2" => Some(crate::ArchiveFormat::TarBz2),
        _ => None,
    }
}

fn pack_zip(archive: PathBuf, files: Vec<PathBuf>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let file = File::create(&archive).map_err(|e| EzError::General(format!("Cannot create archive: {}", e)))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let mut packed_files = Vec::new();
    for path in files {
        if path.is_file() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            zip.start_file(name.clone(), options).map_err(|e| EzError::General(format!("Zip error: {}", e)))?;
            let contents = std::fs::read(&path).map_err(|e| EzError::General(format!("Read error: {}", e)))?;
            zip.write_all(&contents).map_err(|e| EzError::General(format!("Write error: {}", e)))?;
            packed_files.push(name);
        } else if path.is_dir() {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let relative_path = entry.path().strip_prefix(&path.parent().unwrap_or(&path))
                        .unwrap_or(entry.path());
                    let rel_str = relative_path.to_string_lossy().to_string();
                    zip.start_file(rel_str.clone(), options).map_err(|e| EzError::General(format!("Zip error: {}", e)))?;
                    let contents = std::fs::read(entry.path()).map_err(|e| EzError::General(format!("Read error: {}", e)))?;
                    zip.write_all(&contents).map_err(|e| EzError::General(format!("Write error: {}", e)))?;
                    packed_files.push(rel_str);
                }
            }
        }
    }

    zip.finish().map_err(|e| EzError::General(format!("Zip finish error: {}", e)))?;

    if !ctx.json {
        println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    }

    Ok(CommandOutput::new("pack", serde_json::json!({
        "archive": archive.display().to_string(),
        "files": packed_files,
        "format": "zip",
    })))
}

fn pack_tar(archive: PathBuf, files: Vec<PathBuf>, compression: Option<flate2::Compression>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let file = File::create(&archive).map_err(|e| EzError::General(format!("Cannot create archive: {}", e)))?;
    let fmt_name = if compression.is_some() { "tar.gz" } else { "tar" };

    let enc: Box<dyn Write> = match compression {
        Some(level) => Box::new(flate2::write::GzEncoder::new(file, level)),
        None => Box::new(file),
    };

    let mut tar = tar::Builder::new(enc);
    let mut packed_files = Vec::new();

    for path in files {
        let name = path.display().to_string();
        if path.is_file() {
            tar.append_path(&path).map_err(|e| EzError::General(format!("Tar error: {}", e)))?;
            packed_files.push(name);
        } else if path.is_dir() {
            tar.append_dir_all(path.file_name().unwrap(), &path)
                .map_err(|e| EzError::General(format!("Tar error: {}", e)))?;
            packed_files.push(name);
        }
    }

    tar.finish().map_err(|e| EzError::General(format!("Tar finish error: {}", e)))?;

    if !ctx.json {
        println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    }

    Ok(CommandOutput::new("pack", serde_json::json!({
        "archive": archive.display().to_string(),
        "files": packed_files,
        "format": fmt_name,
    })))
}

fn pack_tar_bz2(archive: PathBuf, files: Vec<PathBuf>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let file = File::create(&archive).map_err(|e| EzError::General(format!("Cannot create archive: {}", e)))?;
    let enc = bzip2::write::BzEncoder::new(file, bzip2::Compression::default());
    let mut tar = tar::Builder::new(enc);
    let mut packed_files = Vec::new();

    for path in files {
        let name = path.display().to_string();
        if path.is_file() {
            tar.append_path(&path).map_err(|e| EzError::General(format!("Tar error: {}", e)))?;
            packed_files.push(name);
        } else if path.is_dir() {
            tar.append_dir_all(path.file_name().unwrap(), &path)
                .map_err(|e| EzError::General(format!("Tar error: {}", e)))?;
            packed_files.push(name);
        }
    }

    tar.finish().map_err(|e| EzError::General(format!("Tar finish error: {}", e)))?;

    if !ctx.json {
        println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    }

    Ok(CommandOutput::new("pack", serde_json::json!({
        "archive": archive.display().to_string(),
        "files": packed_files,
        "format": "tar.bz2",
    })))
}
