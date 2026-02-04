use serde::Serialize;
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

#[derive(Serialize)]
pub struct CommandSchema {
    pub name: &'static str,
    pub aliases: Vec<&'static str>,
    pub description: &'static str,
    pub args: Vec<ArgSchema>,
    pub flags: Vec<FlagSchema>,
}

#[derive(Serialize)]
pub struct ArgSchema {
    pub name: &'static str,
    pub description: &'static str,
    pub required: bool,
    pub default: Option<&'static str>,
    pub multiple: bool,
}

#[derive(Serialize)]
pub struct FlagSchema {
    pub short: Option<char>,
    pub long: &'static str,
    pub description: &'static str,
    pub takes_value: bool,
}

fn build_registry() -> Vec<CommandSchema> {
    vec![
        CommandSchema {
            name: "list",
            aliases: vec!["ls"],
            description: "List files and folders",
            args: vec![ArgSchema { name: "path", description: "Path to list", required: false, default: Some("."), multiple: false }],
            flags: vec![
                FlagSchema { short: Some('a'), long: "all", description: "Show all files including hidden", takes_value: false },
                FlagSchema { short: Some('d'), long: "details", description: "Show details (size, modified date)", takes_value: false },
                FlagSchema { short: Some('t'), long: "time", description: "Sort by modified time", takes_value: false },
                FlagSchema { short: Some('s'), long: "size", description: "Sort by size", takes_value: false },
            ],
        },
        CommandSchema {
            name: "show",
            aliases: vec!["cat"],
            description: "Show contents of a file",
            args: vec![ArgSchema { name: "file", description: "File to show", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('n'), long: "numbers", description: "Show line numbers", takes_value: false },
                FlagSchema { short: Some('f'), long: "first", description: "Only show first N lines", takes_value: true },
                FlagSchema { short: Some('l'), long: "last", description: "Only show last N lines", takes_value: true },
            ],
        },
        CommandSchema {
            name: "peek",
            aliases: vec![],
            description: "Peek at first/last lines of a file",
            args: vec![ArgSchema { name: "file", description: "File to peek at", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('l'), long: "lines", description: "Number of lines to show", takes_value: true },
                FlagSchema { short: Some('t'), long: "tail", description: "Show last lines instead of first", takes_value: false },
            ],
        },
        CommandSchema {
            name: "find",
            aliases: vec![],
            description: "Find files or text",
            args: vec![
                ArgSchema { name: "pattern", description: "What to find", required: true, default: None, multiple: false },
                ArgSchema { name: "path", description: "Where to look", required: false, default: Some("."), multiple: false },
            ],
            flags: vec![
                FlagSchema { short: Some('i'), long: "inside", description: "Find in file contents", takes_value: false },
                FlagSchema { short: Some('c'), long: "ignore-case", description: "Case insensitive search", takes_value: false },
                FlagSchema { short: Some('n'), long: "line-numbers", description: "Show line numbers for content matches", takes_value: false },
            ],
        },
        CommandSchema {
            name: "search",
            aliases: vec!["grep"],
            description: "Search for text in files recursively",
            args: vec![
                ArgSchema { name: "pattern", description: "Pattern to search for", required: true, default: None, multiple: false },
                ArgSchema { name: "path", description: "Where to search", required: false, default: Some("."), multiple: false },
            ],
            flags: vec![
                FlagSchema { short: Some('c'), long: "context", description: "Number of context lines", takes_value: true },
            ],
        },
        CommandSchema {
            name: "permissions",
            aliases: vec!["perms"],
            description: "Show file permissions",
            args: vec![ArgSchema { name: "path", description: "File or directory to check", required: true, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "tree",
            aliases: vec![],
            description: "Show directory tree structure",
            args: vec![ArgSchema { name: "path", description: "Root directory", required: false, default: Some("."), multiple: false }],
            flags: vec![
                FlagSchema { short: Some('d'), long: "depth", description: "Maximum depth to display", takes_value: true },
            ],
        },
        CommandSchema {
            name: "env",
            aliases: vec![],
            description: "Show environment variables",
            args: vec![ArgSchema { name: "pattern", description: "Filter by pattern", required: false, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "network",
            aliases: vec!["net"],
            description: "Show network interfaces and IPs",
            args: vec![],
            flags: vec![],
        },
        CommandSchema {
            name: "ports",
            aliases: vec![],
            description: "Show listening ports",
            args: vec![ArgSchema { name: "port", description: "Filter by port number", required: false, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "watch",
            aliases: vec![],
            description: "Watch a file or command for changes",
            args: vec![ArgSchema { name: "target", description: "File path or command to watch", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('i'), long: "interval", description: "Check interval in seconds", takes_value: true },
            ],
        },
        CommandSchema {
            name: "disk",
            aliases: vec![],
            description: "Show disk I/O stats",
            args: vec![],
            flags: vec![],
        },
        CommandSchema {
            name: "replace",
            aliases: vec!["sed"],
            description: "Find and replace in files",
            args: vec![
                ArgSchema { name: "old", description: "Text to find", required: true, default: None, multiple: false },
                ArgSchema { name: "new", description: "Text to replace with", required: true, default: None, multiple: false },
                ArgSchema { name: "file", description: "File to modify", required: true, default: None, multiple: false },
            ],
            flags: vec![
                FlagSchema { short: Some('a'), long: "all", description: "Replace all occurrences", takes_value: false },
            ],
        },
        CommandSchema {
            name: "copy",
            aliases: vec!["cp"],
            description: "Copy files or folders",
            args: vec![
                ArgSchema { name: "from", description: "Source file or folder", required: true, default: None, multiple: false },
                ArgSchema { name: "to", description: "Destination", required: true, default: None, multiple: false },
            ],
            flags: vec![
                FlagSchema { short: Some('r'), long: "recursive", description: "Copy folders recursively", takes_value: false },
                FlagSchema { short: Some('p'), long: "progress", description: "Show progress", takes_value: false },
                FlagSchema { short: None, long: "if-not-exists", description: "Skip if destination exists", takes_value: false },
            ],
        },
        CommandSchema {
            name: "move",
            aliases: vec!["mv"],
            description: "Move or rename files",
            args: vec![
                ArgSchema { name: "from", description: "Source file or folder", required: true, default: None, multiple: false },
                ArgSchema { name: "to", description: "Destination", required: true, default: None, multiple: false },
            ],
            flags: vec![
                FlagSchema { short: None, long: "if-not-exists", description: "Skip if destination exists", takes_value: false },
            ],
        },
        CommandSchema {
            name: "remove",
            aliases: vec!["rm"],
            description: "Remove files or folders",
            args: vec![ArgSchema { name: "paths", description: "Files or folders to remove", required: true, default: None, multiple: true }],
            flags: vec![
                FlagSchema { short: Some('r'), long: "recursive", description: "Remove folders and their contents", takes_value: false },
                FlagSchema { short: Some('f'), long: "force", description: "Force removal without asking", takes_value: false },
            ],
        },
        CommandSchema {
            name: "create-folder",
            aliases: vec!["mkdir"],
            description: "Create a folder",
            args: vec![ArgSchema { name: "paths", description: "Folder paths to create", required: true, default: None, multiple: true }],
            flags: vec![
                FlagSchema { short: Some('p'), long: "parents", description: "Create parent folders if needed", takes_value: false },
                FlagSchema { short: None, long: "if-not-exists", description: "Return success if already exists", takes_value: false },
            ],
        },
        CommandSchema {
            name: "create-file",
            aliases: vec!["touch"],
            description: "Create an empty file",
            args: vec![ArgSchema { name: "paths", description: "File paths to create", required: true, default: None, multiple: true }],
            flags: vec![
                FlagSchema { short: None, long: "if-not-exists", description: "Return success without truncating if exists", takes_value: false },
            ],
        },
        CommandSchema {
            name: "where",
            aliases: vec!["here"],
            description: "Show current location",
            args: vec![],
            flags: vec![],
        },
        CommandSchema {
            name: "size",
            aliases: vec!["usage"],
            description: "Show folder size",
            args: vec![ArgSchema { name: "path", description: "Path to check", required: false, default: Some("."), multiple: false }],
            flags: vec![
                FlagSchema { short: Some('d'), long: "detailed", description: "Show detailed breakdown", takes_value: false },
            ],
        },
        CommandSchema {
            name: "running",
            aliases: vec!["ps"],
            description: "Show running programs",
            args: vec![],
            flags: vec![
                FlagSchema { short: Some('a'), long: "all", description: "Show all processes", takes_value: false },
                FlagSchema { short: Some('f'), long: "filter", description: "Filter by name", takes_value: true },
            ],
        },
        CommandSchema {
            name: "stop",
            aliases: vec!["end"],
            description: "Stop a running program",
            args: vec![ArgSchema { name: "target", description: "Process ID or name to stop", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('f'), long: "force", description: "Force stop immediately", takes_value: false },
            ],
        },
        CommandSchema {
            name: "download",
            aliases: vec!["fetch"],
            description: "Download from internet",
            args: vec![ArgSchema { name: "url", description: "URL to download", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('s'), long: "save", description: "Save as this filename", takes_value: true },
                FlagSchema { short: Some('p'), long: "progress", description: "Show progress", takes_value: false },
            ],
        },
        CommandSchema {
            name: "pack",
            aliases: vec!["compress"],
            description: "Pack files into archive",
            args: vec![
                ArgSchema { name: "archive", description: "Archive file to create", required: true, default: None, multiple: false },
                ArgSchema { name: "files", description: "Files or folders to pack", required: true, default: None, multiple: true },
            ],
            flags: vec![
                FlagSchema { short: Some('f'), long: "format", description: "Compression format", takes_value: true },
            ],
        },
        CommandSchema {
            name: "unpack",
            aliases: vec!["extract"],
            description: "Unpack an archive",
            args: vec![ArgSchema { name: "archive", description: "Archive file to unpack", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('t'), long: "to", description: "Where to unpack", takes_value: true },
            ],
        },
        CommandSchema {
            name: "space",
            aliases: vec!["diskfree"],
            description: "Show disk space",
            args: vec![],
            flags: vec![],
        },
        CommandSchema {
            name: "count",
            aliases: vec![],
            description: "Count lines, words, bytes in files",
            args: vec![ArgSchema { name: "files", description: "Files to count", required: true, default: None, multiple: true }],
            flags: vec![
                FlagSchema { short: Some('l'), long: "lines", description: "Count lines only", takes_value: false },
                FlagSchema { short: Some('w'), long: "words", description: "Count words only", takes_value: false },
                FlagSchema { short: Some('b'), long: "bytes", description: "Count bytes only", takes_value: false },
            ],
        },
        CommandSchema {
            name: "sort",
            aliases: vec![],
            description: "Sort lines in files",
            args: vec![ArgSchema { name: "file", description: "File to sort", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: Some('r'), long: "reverse", description: "Sort in reverse order", takes_value: false },
                FlagSchema { short: Some('n'), long: "numeric", description: "Sort numerically", takes_value: false },
                FlagSchema { short: Some('u'), long: "unique", description: "Remove duplicate lines", takes_value: false },
            ],
        },
        CommandSchema {
            name: "compare",
            aliases: vec!["diff"],
            description: "Show differences between files",
            args: vec![
                ArgSchema { name: "file1", description: "First file", required: true, default: None, multiple: false },
                ArgSchema { name: "file2", description: "Second file", required: true, default: None, multiple: false },
            ],
            flags: vec![
                FlagSchema { short: Some('s'), long: "side-by-side", description: "Show side by side", takes_value: false },
            ],
        },
        CommandSchema {
            name: "make-runnable",
            aliases: vec![],
            description: "Make file executable",
            args: vec![ArgSchema { name: "file", description: "File to make executable", required: true, default: None, multiple: false }],
            flags: vec![
                FlagSchema { short: None, long: "if-not-exists", description: "Skip if already executable", takes_value: false },
            ],
        },
        CommandSchema {
            name: "explain",
            aliases: vec![],
            description: "Explain any Unix command in plain English",
            args: vec![ArgSchema { name: "command", description: "The command to explain", required: true, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "chain",
            aliases: vec![],
            description: "Build a Unix pipeline from natural language",
            args: vec![ArgSchema { name: "query", description: "What you want to do in plain English", required: true, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "help-me",
            aliases: vec!["examples"],
            description: "Show command help and examples",
            args: vec![ArgSchema { name: "command", description: "Command to get help for", required: false, default: None, multiple: false }],
            flags: vec![],
        },
        CommandSchema {
            name: "schema",
            aliases: vec![],
            description: "Show machine-readable command schema",
            args: vec![ArgSchema { name: "command", description: "Show schema for specific command", required: false, default: None, multiple: false }],
            flags: vec![],
        },
    ]
}

pub fn execute(command: Option<String>, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let registry = build_registry();

    let data = if let Some(cmd_name) = command {
        let schema = registry
            .into_iter()
            .find(|s| s.name == cmd_name || s.aliases.contains(&cmd_name.as_str()));

        match schema {
            Some(s) => serde_json::to_value(&s).map_err(|e| EzError::General(e.to_string()))?,
            None => return Err(EzError::NotFound(format!("Unknown command: {}", cmd_name))),
        }
    } else {
        serde_json::to_value(&registry).map_err(|e| EzError::General(e.to_string()))?
    };

    // Schema always outputs JSON, even without --json flag.
    // When --json is set, output_result handles it via the envelope.
    if !ctx.json {
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    }

    Ok(CommandOutput::new("schema", data))
}
