use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(paths: Vec<PathBuf>, recursive: bool, force: bool) -> Result<(), String> {
    for path in paths {
        if !path.exists() {
            if force {
                continue;
            }
            return Err(format!("'{}' does not exist", path.display()));
        }

        let is_dir = path.is_dir();

        if is_dir && !recursive {
            return Err(format!(
                "'{}' is a folder. Use --recursive to remove folders",
                path.display()
            ));
        }

        if is_dir {
            if !force {
                print!("Remove folder '{}' and all contents? [y/N] ", path.display());
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Skipped '{}'", path.display());
                    continue;
                }
            }
            fs::remove_dir_all(&path).map_err(|e| format!("Cannot remove: {}", e))?;
        } else {
            fs::remove_file(&path).map_err(|e| format!("Cannot remove: {}", e))?;
        }

        println!("{} Removed '{}'", "✓".green(), path.display());
    }

    Ok(())
}
