mod commands;
mod utils;

use commands::*;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ez")]
#[command(about = "Easy Unix - User-friendly command line tools")]
#[command(version = "0.1.0")]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

    /// Find files or text (like find/grep)
    #[command(name = "find", alias = "search")]
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
    },

    /// Move or rename files
    #[command(name = "move", alias = "mv")]
    Move {
        /// Source file or folder
        from: PathBuf,
        /// Destination
        to: PathBuf,
    },

    /// Remove files or folders (like rm)
    #[command(name = "remove", alias = "rm")]
    Remove {
        /// Files or folders to remove
        #[arg(required = true)]
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
        #[arg(required = true)]
        paths: Vec<PathBuf>,
        /// Create parent folders if needed
        #[arg(short, long)]
        parents: bool,
    },

    /// Create an empty file (like touch)
    #[command(name = "create-file", alias = "touch")]
    CreateFile {
        /// File path to create
        #[arg(required = true)]
        paths: Vec<PathBuf>,
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
        #[arg(required = true)]
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
    #[command(name = "space", alias = "disk")]
    Space,

    /// Count lines, words, bytes in files (like wc)
    #[command(name = "count")]
    Count {
        /// Files to count
        #[arg(required = true)]
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
    },

    /// Show command help and examples
    #[command(name = "help-me", alias = "examples")]
    HelpMe {
        /// Command to get help for
        command: Option<String>,
    },
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    TarBz2,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::List { path, all, details, time, size } => {
            list::execute(path, all, details, time, size)
        }
        Commands::Show { file, numbers, first, last } => {
            show::execute(file, numbers, first, last)
        }
        Commands::Find { pattern, path, inside, ignore_case, line_numbers } => {
            find::execute(pattern, path, inside, ignore_case, line_numbers)
        }
        Commands::Copy { from, to, recursive, progress } => {
            copy::execute(from, to, recursive, progress)
        }
        Commands::Move { from, to } => {
            r#move::execute(from, to)
        }
        Commands::Remove { paths, recursive, force } => {
            remove::execute(paths, recursive, force)
        }
        Commands::CreateFolder { paths, parents } => {
            create_folder::execute(paths, parents)
        }
        Commands::CreateFile { paths } => {
            create_file::execute(paths)
        }
        Commands::Where => {
            r#where::execute()
        }
        Commands::Size { path, detailed } => {
            size::execute(path, detailed)
        }
        Commands::Running { all, filter } => {
            running::execute(all, filter)
        }
        Commands::Stop { target, force } => {
            stop::execute(target, force)
        }
        Commands::Download { url, save, progress } => {
            download::execute(url, save, progress)
        }
        Commands::Pack { archive, files, format } => {
            pack::execute(archive, files, format)
        }
        Commands::Unpack { archive, to } => {
            unpack::execute(archive, to)
        }
        Commands::Space => {
            space::execute()
        }
        Commands::Count { files, lines, words, bytes } => {
            count::execute(files, lines, words, bytes)
        }
        Commands::Sort { file, reverse, numeric, unique } => {
            sort::execute(file, reverse, numeric, unique)
        }
        Commands::Compare { file1, file2, side_by_side } => {
            compare::execute(file1, file2, side_by_side)
        }
        Commands::MakeRunnable { file } => {
            make_runnable::execute(file)
        }
        Commands::HelpMe { command } => {
            help_me::execute(command)
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
