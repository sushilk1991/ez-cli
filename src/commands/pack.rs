use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use zip::write::FileOptions;

pub fn execute(
    archive: PathBuf,
    files: Vec<PathBuf>,
    format: Option<crate::ArchiveFormat>,
) -> Result<(), String> {
    let format = format.or_else(|| detect_format(&archive));

    match format {
        Some(crate::ArchiveFormat::Zip) => pack_zip(archive, files),
        Some(crate::ArchiveFormat::Tar) => pack_tar(archive, files, None),
        Some(crate::ArchiveFormat::TarGz) => pack_tar(archive, files, Some(flate2::Compression::default())),
        Some(crate::ArchiveFormat::TarBz2) => pack_tar_bz2(archive, files),
        None => Err("Cannot detect archive format from extension. Use --format".to_string()),
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

fn pack_zip(archive: PathBuf, files: Vec<PathBuf>) -> Result<(), String> {
    let file = File::create(&archive).map_err(|e| format!("Cannot create archive: {}", e))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for path in files {
        if path.is_file() {
            let name = path.file_name().unwrap().to_string_lossy();
            zip.start_file(name.to_string(), options)
                .map_err(|e| format!("Zip error: {}", e))?;
            let contents = std::fs::read(&path).map_err(|e| format!("Read error: {}", e))?;
            zip.write_all(&contents).map_err(|e| format!("Write error: {}", e))?;
        } else if path.is_dir() {
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let relative_path = entry.path().strip_prefix(&path.parent().unwrap_or(&path))
                        .unwrap_or(entry.path());
                    zip.start_file(relative_path.to_string_lossy().to_string(), options)
                        .map_err(|e| format!("Zip error: {}", e))?;
                    let contents = std::fs::read(entry.path()).map_err(|e| format!("Read error: {}", e))?;
                    zip.write_all(&contents).map_err(|e| format!("Write error: {}", e))?;
                }
            }
        }
    }

    zip.finish().map_err(|e| format!("Zip finish error: {}", e))?;
    println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    Ok(())
}

fn pack_tar(archive: PathBuf, files: Vec<PathBuf>, compression: Option<flate2::Compression>) -> Result<(), String> {
    let file = File::create(&archive).map_err(|e| format!("Cannot create archive: {}", e))?;
    
    let enc: Box<dyn Write> = match compression {
        Some(level) => Box::new(flate2::write::GzEncoder::new(file, level)),
        None => Box::new(file),
    };
    
    let mut tar = tar::Builder::new(enc);

    for path in files {
        if path.is_file() {
            tar.append_path(&path).map_err(|e| format!("Tar error: {}", e))?;
        } else if path.is_dir() {
            tar.append_dir_all(path.file_name().unwrap(), &path)
                .map_err(|e| format!("Tar error: {}", e))?;
        }
    }

    tar.finish().map_err(|e| format!("Tar finish error: {}", e))?;
    println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    Ok(())
}

fn pack_tar_bz2(archive: PathBuf, files: Vec<PathBuf>) -> Result<(), String> {
    let file = File::create(&archive).map_err(|e| format!("Cannot create archive: {}", e))?;
    let enc = bzip2::write::BzEncoder::new(file, bzip2::Compression::default());
    let mut tar = tar::Builder::new(enc);

    for path in files {
        if path.is_file() {
            tar.append_path(&path).map_err(|e| format!("Tar error: {}", e))?;
        } else if path.is_dir() {
            tar.append_dir_all(path.file_name().unwrap(), &path)
                .map_err(|e| format!("Tar error: {}", e))?;
        }
    }

    tar.finish().map_err(|e| format!("Tar finish error: {}", e))?;
    println!("{} Created archive {}", "✓".green(), archive.display().to_string().cyan());
    Ok(())
}
