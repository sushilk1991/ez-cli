mod commands;
mod context;
mod output;
mod utils;

use commands::*;
use context::CommandContext;
use output::output_result;
use clap::{Parser, Subcommand};
use std::io::IsTerminal;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ez")]
#[command(about = "Easy Unix - User-friendly command line tools")]
#[command(version = "0.1.0")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Output results as JSON for AI agents
    #[arg(long, global = true)]
    json: bool,
    /// Skip confirmation prompts (answer yes to everything)
    #[arg(long, alias = "no-confirm", global = true)]
    yes: bool,
    /// Preview what would happen without making changes
    #[arg(long, global = true)]
    dry_run: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List files and folders (like ls)
    #[command(name = "list", alias = "ls")]
    List {
        /// Path to list (defaults to current folder)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show all files including hidden
        #[arg(short, long)]
        all: bool,
        /// Show details (size, modified date)
        #[arg(short, long)]
        details: bool,
        /// Sort by modified time
        #[arg(short, long)]
        time: bool,
        /// Sort by size
        #[arg(short, long)]
        size: bool,
    },

    /// Show contents of a file (like cat)
    #[command(name = "show", alias = "cat")]
    Show {
        /// File to show
        file: PathBuf,
        /// Show line numbers
        #[arg(short, long)]
        numbers: bool,
        /// Only show first N lines
        #[arg(short, long, value_name = "N")]
        first: Option<usize>,
        /// Only show last N lines
        #[arg(short, long, value_name = "N")]
        last: Option<usize>,
    },

    /// Peek at first/last lines of a file (like head/tail)
    #[command(name = "peek")]
    Peek {
        /// File to peek at
        file: PathBuf,
        /// Number of lines to show
        #[arg(short, long, default_value = "10")]
        lines: usize,
        /// Show last lines instead of first
        #[arg(short, long)]
        tail: bool,
    },

    /// Find files or text (like find/grep)
    #[command(name = "find")]
    Find {
        /// What to find
        pattern: String,
        /// Where to look (defaults to current folder)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Find in file contents (like grep)
        #[arg(short, long)]
        inside: bool,
        /// Case insensitive search
        #[arg(short, long)]
        ignore_case: bool,
        /// Show line numbers for content matches
        #[arg(short, long)]
        line_numbers: bool,
    },

    /// Search for text in files recursively (like grep -r)
    #[command(name = "search", alias = "grep")]
    Search {
        /// Pattern to search for
        pattern: String,
        /// Where to search (defaults to current folder)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Number of context lines to show
        #[arg(short, long, default_value = "2")]
        context: usize,
    },

    /// Show file permissions (like ls -la)
    #[command(name = "permissions", alias = "perms")]
    Permissions {
        /// File or directory to check
        path: PathBuf,
    },

    /// Show directory tree structure
    #[command(name = "tree")]
    Tree {
        /// Root directory (defaults to current)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Maximum depth to display
        #[arg(short, long, default_value = "3")]
        depth: usize,
    },

    /// Show environment variables (like env/printenv)
    #[command(name = "env")]
    Env {
        /// Filter by pattern
        pattern: Option<String>,
    },

    /// Show network interfaces and IPs (like ifconfig/ip)
    #[command(name = "network", alias = "net")]
    Network,

    /// Show listening ports (like netstat/ss)
    #[command(name = "ports")]
    Ports {
        /// Filter by port number
        port: Option<String>,
    },

    /// Watch a file or command for changes
    #[command(name = "watch")]
    Watch {
        /// File path or command to watch
        target: String,
        /// Check interval in seconds
        #[arg(short, long, default_value = "2")]
        interval: u64,
    },

    /// Show disk I/O stats (like iostat)
    #[command(name = "disk")]
    Disk,

    /// Find and replace in files (like sed)
    #[command(name = "replace", alias = "sed")]
    Replace {
        /// Text to find
        old: String,
        /// Text to replace with
        new: String,
        /// File to modify
        file: PathBuf,
        /// Replace all occurrences
        #[arg(short, long)]
        all: bool,
    },

    /// Copy files or folders
    #[command(name = "copy", alias = "cp")]
    Copy {
        /// Source file or folder
        from: PathBuf,
        /// Destination
        to: PathBuf,
        /// Copy folders recursively
        #[arg(short, long)]
        recursive: bool,
        /// Show progress
        #[arg(short, long)]
        progress: bool,
        /// Skip if destination already exists
        #[arg(long)]
        if_not_exists: bool,
    },

    /// Move or rename files
    #[command(name = "move", alias = "mv")]
    Move {
        /// Source file or folder
        from: PathBuf,
        /// Destination
        to: PathBuf,
        /// Skip if destination already exists
        #[arg(long)]
        if_not_exists: bool,
    },

    /// Remove files or folders (like rm)
    #[command(name = "remove", alias = "rm")]
    Remove {
        /// Files or folders to remove
        paths: Vec<PathBuf>,
        /// Remove folders and their contents
        #[arg(short, long)]
        recursive: bool,
        /// Force removal without asking
        #[arg(short, long)]
        force: bool,
    },

    /// Create a folder (like mkdir)
    #[command(name = "create-folder", alias = "mkdir")]
    CreateFolder {
        /// Folder path to create
        paths: Vec<PathBuf>,
        /// Create parent folders if needed
        #[arg(short, long)]
        parents: bool,
        /// Skip if folder already exists
        #[arg(long)]
        if_not_exists: bool,
    },

    /// Create an empty file (like touch)
    #[command(name = "create-file", alias = "touch")]
    CreateFile {
        /// File path to create
        paths: Vec<PathBuf>,
        /// Skip if file already exists
        #[arg(long)]
        if_not_exists: bool,
    },

    /// Show current location (like pwd)
    #[command(name = "where", alias = "here")]
    Where,

    /// Show folder size (like du)
    #[command(name = "size", alias = "usage")]
    Size {
        /// Path to check (defaults to current folder)
        #[arg(default_value = ".")]
        path: PathBuf,
        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },

    /// Show running programs (like ps)
    #[command(name = "running", alias = "ps")]
    Running {
        /// Show all processes
        #[arg(short, long)]
        all: bool,
        /// Filter by name
        #[arg(short, long)]
        filter: Option<String>,
    },

    /// Stop a running program (like kill)
    #[command(name = "stop", alias = "end")]
    Stop {
        /// Process ID or name to stop
        target: String,
        /// Force stop immediately
        #[arg(short, long)]
        force: bool,
    },

    /// Download from internet (like curl/wget)
    #[command(name = "download", alias = "fetch")]
    Download {
        /// URL to download
        url: String,
        /// Save as this filename
        #[arg(short, long)]
        save: Option<String>,
        /// Show progress
        #[arg(short, long)]
        progress: bool,
    },

    /// Pack files into archive (like tar/zip)
    #[command(name = "pack", alias = "compress")]
    Pack {
        /// Archive file to create
        archive: PathBuf,
        /// Files or folders to pack
        files: Vec<PathBuf>,
        /// Compression format (auto-detected from extension)
        #[arg(short, long, value_enum)]
        format: Option<ArchiveFormat>,
    },

    /// Unpack an archive (like tar/zip)
    #[command(name = "unpack", alias = "extract")]
    Unpack {
        /// Archive file to unpack
        archive: PathBuf,
        /// Where to unpack (defaults to current folder)
        #[arg(short, long)]
        to: Option<PathBuf>,
    },

    /// Show disk space (like df)
    #[command(name = "space", alias = "diskfree")]
    Space,

    /// Count lines, words, bytes in files (like wc)
    #[command(name = "count")]
    Count {
        /// Files to count
        files: Vec<PathBuf>,
        /// Count lines only
        #[arg(short, long)]
        lines: bool,
        /// Count words only
        #[arg(short, long)]
        words: bool,
        /// Count bytes only
        #[arg(short, long)]
        bytes: bool,
    },

    /// Sort lines in files (like sort)
    #[command(name = "sort")]
    Sort {
        /// File to sort
        file: PathBuf,
        /// Sort in reverse order
        #[arg(short, long)]
        reverse: bool,
        /// Sort numerically
        #[arg(short, long)]
        numeric: bool,
        /// Remove duplicate lines
        #[arg(short, long)]
        unique: bool,
    },

    /// Show differences between files (like diff)
    #[command(name = "compare", alias = "diff")]
    Compare {
        /// First file
        file1: PathBuf,
        /// Second file
        file2: PathBuf,
        /// Show side by side
        #[arg(short, long)]
        side_by_side: bool,
    },

    /// Make file executable (like chmod +x)
    #[command(name = "make-runnable")]
    MakeRunnable {
        /// File to make executable
        file: PathBuf,
        /// Skip if already executable
        #[arg(long)]
        if_not_exists: bool,
    },

    /// Explain any Unix command in plain English
    #[command(name = "explain")]
    Explain {
        /// The command to explain
        command: String,
    },

    /// Build a Unix pipeline from natural language
    #[command(name = "chain")]
    Chain {
        /// What you want to do in plain English
        query: String,
    },

    /// Show command help and examples
    #[command(name = "help-me", alias = "examples")]
    HelpMe {
        /// Command to get help for
        command: Option<String>,
    },

    /// Show machine-readable schema for commands
    #[command(name = "schema")]
    Schema {
        /// Show schema for a specific command
        command: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    TarBz2,
}

fn main() {
    let cli = Cli::parse();

    // Auto-strip ANSI colors when stdout is not a terminal (piped output)
    if !std::io::stdout().is_terminal() {
        colored::control::set_override(false);
    }

    let ctx = CommandContext::new(cli.json, cli.yes, cli.dry_run);

    // For commands that accept Vec<PathBuf>, read from stdin if empty and stdin is piped
    let command_name: &'static str = match &cli.command {
        Commands::List { .. } => "list",
        Commands::Show { .. } => "show",
        Commands::Peek { .. } => "peek",
        Commands::Find { .. } => "find",
        Commands::Search { .. } => "search",
        Commands::Permissions { .. } => "permissions",
        Commands::Tree { .. } => "tree",
        Commands::Env { .. } => "env",
        Commands::Network => "network",
        Commands::Ports { .. } => "ports",
        Commands::Watch { .. } => "watch",
        Commands::Disk => "disk",
        Commands::Replace { .. } => "replace",
        Commands::Copy { .. } => "copy",
        Commands::Move { .. } => "move",
        Commands::Remove { .. } => "remove",
        Commands::CreateFolder { .. } => "create-folder",
        Commands::CreateFile { .. } => "create-file",
        Commands::Where => "where",
        Commands::Size { .. } => "size",
        Commands::Running { .. } => "running",
        Commands::Stop { .. } => "stop",
        Commands::Download { .. } => "download",
        Commands::Pack { .. } => "pack",
        Commands::Unpack { .. } => "unpack",
        Commands::Space => "space",
        Commands::Count { .. } => "count",
        Commands::Sort { .. } => "sort",
        Commands::Compare { .. } => "compare",
        Commands::MakeRunnable { .. } => "make-runnable",
        Commands::Explain { .. } => "explain",
        Commands::Chain { .. } => "chain",
        Commands::HelpMe { .. } => "help-me",
        Commands::Schema { .. } => "schema",
    };

    let result = match cli.command {
        Commands::List { path, all, details, time, size } => {
            list::execute(path, all, details, time, size, &ctx)
        }
        Commands::Show { file, numbers, first, last } => {
            show::execute(file, numbers, first, last, &ctx)
        }
        Commands::Peek { file, lines, tail } => {
            peek::execute(file, lines, tail, &ctx)
        }
        Commands::Find { pattern, path, inside, ignore_case, line_numbers } => {
            find::execute(pattern, path, inside, ignore_case, line_numbers, &ctx)
        }
        Commands::Search { pattern, path, context } => {
            search::execute(pattern, path, context, &ctx)
        }
        Commands::Permissions { path } => {
            permissions::execute(path, &ctx)
        }
        Commands::Tree { path, depth } => {
            tree::execute(path, depth, &ctx)
        }
        Commands::Env { pattern } => {
            env::execute(pattern, &ctx)
        }
        Commands::Network => {
            network::execute(&ctx)
        }
        Commands::Ports { port } => {
            ports::execute(port, &ctx)
        }
        Commands::Watch { target, interval } => {
            watch::execute(target, interval, &ctx)
        }
        Commands::Disk => {
            disk::execute(&ctx)
        }
        Commands::Replace { old, new, file, all } => {
            replace::execute(old, new, file, all, &ctx)
        }
        Commands::Copy { from, to, recursive, progress, if_not_exists } => {
            copy::execute(from, to, recursive, progress, if_not_exists, &ctx)
        }
        Commands::Move { from, to, if_not_exists } => {
            r#move::execute(from, to, if_not_exists, &ctx)
        }
        Commands::Remove { mut paths, recursive, force } => {
            if paths.is_empty() && !ctx.is_stdin_tty {
                paths = utils::read_paths_from_stdin();
            }
            remove::execute(paths, recursive, force, &ctx)
        }
        Commands::CreateFolder { mut paths, parents, if_not_exists } => {
            if paths.is_empty() && !ctx.is_stdin_tty {
                paths = utils::read_paths_from_stdin();
            }
            create_folder::execute(paths, parents, if_not_exists, &ctx)
        }
        Commands::CreateFile { mut paths, if_not_exists } => {
            if paths.is_empty() && !ctx.is_stdin_tty {
                paths = utils::read_paths_from_stdin();
            }
            create_file::execute(paths, if_not_exists, &ctx)
        }
        Commands::Where => {
            r#where::execute(&ctx)
        }
        Commands::Size { path, detailed } => {
            size::execute(path, detailed, &ctx)
        }
        Commands::Running { all, filter } => {
            running::execute(all, filter, &ctx)
        }
        Commands::Stop { target, force } => {
            stop::execute(target, force, &ctx)
        }
        Commands::Download { url, save, progress } => {
            download::execute(url, save, progress, &ctx)
        }
        Commands::Pack { archive, mut files, format } => {
            if files.is_empty() && !ctx.is_stdin_tty {
                files = utils::read_paths_from_stdin();
            }
            pack::execute(archive, files, format, &ctx)
        }
        Commands::Unpack { archive, to } => {
            unpack::execute(archive, to, &ctx)
        }
        Commands::Space => {
            space::execute(&ctx)
        }
        Commands::Count { mut files, lines, words, bytes } => {
            if files.is_empty() && !ctx.is_stdin_tty {
                files = utils::read_paths_from_stdin();
            }
            count::execute(files, lines, words, bytes, &ctx)
        }
        Commands::Sort { file, reverse, numeric, unique } => {
            sort::execute(file, reverse, numeric, unique, &ctx)
        }
        Commands::Compare { file1, file2, side_by_side } => {
            compare::execute(file1, file2, side_by_side, &ctx)
        }
        Commands::MakeRunnable { file, if_not_exists } => {
            make_runnable::execute(file, if_not_exists, &ctx)
        }
        Commands::Explain { command } => {
            explain::execute(command, &ctx)
        }
        Commands::Chain { query } => {
            chain::run(&query, &ctx)
        }
        Commands::HelpMe { command } => {
            help_me::execute(command, &ctx)
        }
        Commands::Schema { command } => {
            schema::execute(command, &ctx)
        }
    };

    output_result(ctx.json, command_name, result);
}
