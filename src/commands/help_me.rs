use colored::*;

pub fn execute(command: Option<String>) -> Result<(), String> {
    match command {
        Some(cmd) => show_command_help(&cmd),
        None => show_all_help(),
    }
}

fn show_all_help() {
    println!("{}", "Easy Unix - User-Friendly Command Reference".bold().underline());
    println!();
    
    let commands = vec![
        ("list / ls", "List files and folders", "ez list [path] --all --details --time --size"),
        ("show / cat", "Show file contents", "ez show file.txt --numbers --first 10"),
        ("find / search", "Find files or text", "ez find pattern --inside --ignore-case"),
        ("copy / cp", "Copy files/folders", "ez copy from to --recursive --progress"),
        ("move / mv", "Move or rename files", "ez move old new"),
        ("remove / rm", "Remove files/folders", "ez remove path --recursive --force"),
        ("create-folder", "Create directories", "ez create-folder path --parents"),
        ("create-file", "Create empty files", "ez create-file path"),
        ("where / here", "Show current location", "ez where"),
        ("size / usage", "Show folder sizes", "ez size [path] --detailed"),
        ("running / ps", "Show running processes", "ez running --all --filter name"),
        ("stop / end", "Stop processes", "ez stop pid_or_name --force"),
        ("download", "Download files", "ez download url --save filename --progress"),
        ("pack", "Create archives", "ez pack archive.zip files..."),
        ("unpack", "Extract archives", "ez unpack archive.tar.gz --to folder"),
        ("space / disk", "Show disk space", "ez space"),
        ("count", "Count lines/words/bytes", "ez count files..."),
        ("sort", "Sort file contents", "ez sort file --reverse --numeric --unique"),
        ("compare / diff", "Compare files", "ez compare file1 file2 --side-by-side"),
        ("make-runnable", "Make file executable", "ez make-runnable script.sh"),
    ];

    for (name, desc, example) in commands {
        println!("  {} {}\n     {}\n     Example: {}\n", 
            "•".cyan(), 
            name.bold(),
            desc.dimmed(),
            example.yellow());
    }

    println!("{} Use '{}' for specific command help", "💡".yellow(), "ez help-me command".cyan());
}

fn show_command_help(cmd: &str) -> Result<(), String> {
    let help = match cmd {
        "list" | "ls" => r#"
List files and folders (ls replacement)

USAGE:
    ez list [PATH] [OPTIONS]

OPTIONS:
    -a, --all       Show hidden files (starting with .)
    -d, --details   Show size and modification date
    -t, --time      Sort by modification time
    -s, --size      Sort by file size

EXAMPLES:
    ez list                    # List current directory
    ez list /var/log --details # Detailed listing
    ez list ~ --all --time     # Show all files, sorted by time
"#,
        "show" | "cat" => r#"
Show file contents (cat replacement)

USAGE:
    ez show FILE [OPTIONS]

OPTIONS:
    -n, --numbers   Show line numbers
    -f, --first N   Show only first N lines
    -l, --last N    Show only last N lines

EXAMPLES:
    ez show readme.txt
    ez show log.txt --last 20
    ez show code.py --numbers --first 50
"#,
        "find" | "search" => r#"
Find files or search inside files

USAGE:
    ez find PATTERN [PATH] [OPTIONS]

OPTIONS:
    -i, --inside        Search inside file contents
    -c, --ignore-case   Case insensitive search
    -n, --line-numbers  Show line numbers for matches

EXAMPLES:
    ez find "*.rs"           # Find Rust files
    ez find "TODO" --inside  # Search for TODO in files
    ez find "error" src/ -i -n
"#,
        "copy" | "cp" => r#"
Copy files or folders

USAGE:
    ez copy FROM TO [OPTIONS]

OPTIONS:
    -r, --recursive   Copy folders recursively
    -p, --progress    Show progress bar

EXAMPLES:
    ez copy file.txt backup/
    ez copy folder/ backup/ --recursive --progress
"#,
        "download" | "fetch" => r#"
Download files from the internet

USAGE:
    ez download URL [OPTIONS]

OPTIONS:
    -s, --save NAME   Save with specific filename
    -p, --progress    Show progress bar

EXAMPLES:
    ez download https://example.com/file.zip
    ez download https://example.com/data.json --save mydata.json --progress
"#,
        _ => return Err(format!("No help available for '{}'", cmd)),
    };

    println!("{}", help);
    Ok(())
}
