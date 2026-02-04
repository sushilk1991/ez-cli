use colored::*;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

pub fn execute(command: Option<String>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    match command {
        Some(cmd) => show_command_help(&cmd, ctx),
        None => {
            show_all_help(ctx);
            Ok(CommandOutput::new("help-me", serde_json::json!({ "topic": "all" })))
        }
    }
}

fn show_all_help(ctx: &CommandContext) {
    if ctx.json { return; }

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

fn show_command_help(cmd: &str, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let help = match cmd {
        "list" | "ls" => "List files and folders (ls replacement)\n\nUSAGE:\n    ez list [PATH] [OPTIONS]\n\nOPTIONS:\n    -a, --all       Show hidden files\n    -d, --details   Show size and modification date\n    -t, --time      Sort by modification time\n    -s, --size      Sort by file size",
        "show" | "cat" => "Show file contents (cat replacement)\n\nUSAGE:\n    ez show FILE [OPTIONS]\n\nOPTIONS:\n    -n, --numbers   Show line numbers\n    -f, --first N   Show only first N lines\n    -l, --last N    Show only last N lines",
        "find" | "search" => "Find files or search inside files\n\nUSAGE:\n    ez find PATTERN [PATH] [OPTIONS]\n\nOPTIONS:\n    -i, --inside        Search inside file contents\n    -c, --ignore-case   Case insensitive\n    -n, --line-numbers  Show line numbers",
        "copy" | "cp" => "Copy files or folders\n\nUSAGE:\n    ez copy FROM TO [OPTIONS]\n\nOPTIONS:\n    -r, --recursive   Copy folders recursively\n    -p, --progress    Show progress bar",
        "download" | "fetch" => "Download files from the internet\n\nUSAGE:\n    ez download URL [OPTIONS]\n\nOPTIONS:\n    -s, --save NAME   Save with specific filename\n    -p, --progress    Show progress bar",
        _ => return Err(EzError::NotFound(format!("No help available for '{}'", cmd))),
    };

    if !ctx.json {
        println!("{}", help);
    }

    Ok(CommandOutput::new("help-me", serde_json::json!({
        "topic": cmd,
        "help": help,
    })))
}
