use std::fs;
use std::path::PathBuf;
use colored::*;

pub fn execute(
    old: String,
    new: String,
    file: PathBuf,
    all: bool,
) -> Result<(), String> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        format!("Cannot read '{}': {}", file.display(), e)
    })?;

    let (new_contents, count) = if all {
        let count = contents.matches(&old).count();
        (contents.replace(&old, &new), count)
    } else {
        if let Some(pos) = contents.find(&old) {
            let mut result = contents.clone();
            result.replace_range(pos..pos + old.len(), &new);
            (result, 1)
        } else {
            (contents.clone(), 0)
        }
    };

    if count == 0 {
        println!("{} No matches found for '{}'", "ℹ️".yellow(), old.yellow());
        return Ok(());
    }

    fs::write(&file, new_contents).map_err(|e| {
        format!("Cannot write to '{}': {}", file.display(), e)
    })?;

    println!("{} Replaced {} occurrence(s) of '{}' with '{}' in {}", 
        "✓".green(), 
        count.to_string().cyan().bold(),
        old.yellow(), 
        new.green(),
        file.display().to_string().cyan()
    );

    Ok(())
}
