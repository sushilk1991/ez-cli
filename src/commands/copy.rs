use std::fs;
use std::path::PathBuf;
use colored::*;
use std::io::Write;

pub fn execute(from: PathBuf, to: PathBuf, recursive: bool, progress: bool) -> Result<(), String> {
    if !from.exists() {
        return Err(format!("Source '{}' does not exist", from.display()));
    }

    let is_dir = from.is_dir();

    if is_dir && !recursive {
        return Err(format!(
            "'{}' is a folder. Use --recursive to copy folders",
            from.display()
        ));
    }

    if is_dir {
        copy_dir(&from, &to, progress)?;
    } else {
        copy_file(&from, &to, progress)?;
    }

    println!("{} Copied '{}' to '{}'", "✓".green(), from.display(), to.display());
    Ok(())
}

fn copy_file(from: &PathBuf, to: &PathBuf, progress: bool) -> Result<(), String> {
    if progress {
        let size = fs::metadata(from).map(|m| m.len()).unwrap_or(0);
        let size_str = crate::utils::format_size(size);
        print!("Copying {}... ", size_str.dimmed());
        std::io::stdout().flush().unwrap();
    }

    fs::copy(from, to).map_err(|e| format!("Copy failed: {}", e))?;

    if progress {
        println!("{}", "done".green());
    }

    Ok(())
}

fn copy_dir(from: &PathBuf, to: &PathBuf, progress: bool) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| format!("Cannot create directory: {}", e))?;

    for entry in fs::read_dir(from).map_err(|e| format!("Cannot read directory: {}", e))? {
        let entry = entry.map_err(|e| format!("Read error: {}", e))?;
        let from_path = entry.path();
        let file_name = entry.file_name();
        let to_path = to.join(&file_name);

        if from_path.is_dir() {
            copy_dir(&from_path, &to_path, progress)?;
        } else {
            copy_file(&from_path, &to_path, progress)?;
        }
    }

    Ok(())
}
