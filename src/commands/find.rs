use std::fs;
use std::path::PathBuf;
use colored::*;
use walkdir::WalkDir;
use regex::Regex;

pub fn execute(
    pattern: String,
    path: PathBuf,
    inside: bool,
    ignore_case: bool,
    line_numbers: bool,
) -> Result<(), String> {
    if inside {
        find_in_contents(pattern, path, ignore_case, line_numbers)
    } else {
        find_files(pattern, path)
    }
}

fn find_files(pattern: String, path: PathBuf) -> Result<(), String> {
    let re = Regex::new(&format!("(?i){}", regex::escape(&pattern)))
        .map_err(|e| format!("Invalid pattern: {}", e))?;

    let mut found = 0;

    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy();
        
        if re.is_match(&name) {
            let path_display = entry.path().strip_prefix(&path).unwrap_or(entry.path());
            println!("{}", path_display.display().to_string().green());
            found += 1;
        }
    }

    if found == 0 {
        println!("{} No files matching '{}' found", "ℹ".blue(), pattern);
    } else {
        println!("\n{} Found {} file(s)", "✓".green(), found);
    }

    Ok(())
}

fn find_in_contents(
    pattern: String,
    path: PathBuf,
    ignore_case: bool,
    line_numbers: bool,
) -> Result<(), String> {
    let re = if ignore_case {
        Regex::new(&format!("(?i){}", regex::escape(&pattern)))
    } else {
        Regex::new(&regex::escape(&pattern))
    }.map_err(|e| format!("Invalid pattern: {}", e))?;

    let mut found_files = 0;
    let mut found_matches = 0;

    for entry in WalkDir::new(&path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(contents) = fs::read_to_string(entry.path()) {
            let mut file_matches = vec![];
            
            for (line_num, line) in contents.lines().enumerate() {
                if re.is_match(line) {
                    file_matches.push((line_num + 1, line.to_string()));
                    found_matches += 1;
                }
            }

            if !file_matches.is_empty() {
                found_files += 1;
                let path_display = entry.path().strip_prefix(&path).unwrap_or(entry.path());
                println!("\n{}", path_display.display().to_string().cyan().underline());
                
                for (line_num, line) in file_matches {
                    if line_numbers {
                        let highlighted = re.replace_all(&line, |caps: &regex::Captures| {
                            caps[0].to_string().red().bold().to_string()
                        });
                        println!("  {:>4} │ {}", line_num.to_string().dimmed(), highlighted);
                    } else {
                        let highlighted = re.replace_all(&line, |caps: &regex::Captures| {
                            caps[0].to_string().red().bold().to_string()
                        });
                        println!("  {}", highlighted);
                    }
                }
            }
        }
    }

    if found_matches == 0 {
        println!("{} No matches for '{}' found", "ℹ".blue(), pattern);
    } else {
        println!("\n{} Found {} match(es) in {} file(s)", 
            "✓".green(), found_matches, found_files);
    }

    Ok(())
}
