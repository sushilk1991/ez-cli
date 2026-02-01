use std::fs;
use std::path::PathBuf;

pub fn execute(file: PathBuf, reverse: bool, numeric: bool, unique: bool) -> Result<(), String> {
    let contents = fs::read_to_string(&file).map_err(|e| {
        format!("Cannot read '{}': {}", file.display(), e)
    })?;

    let mut lines: Vec<String> = contents.lines().map(String::from).collect();

    // Sort
    if numeric {
        lines.sort_by(|a, b| {
            let a_num = a.parse::<f64>().unwrap_or(f64::INFINITY);
            let b_num = b.parse::<f64>().unwrap_or(f64::INFINITY);
            a_num.partial_cmp(&b_num).unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        lines.sort();
    }

    // Remove duplicates
    if unique {
        lines.dedup();
    }

    // Reverse
    if reverse {
        lines.reverse();
    }

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
