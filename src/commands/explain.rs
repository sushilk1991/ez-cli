use std::collections::HashMap;
use colored::*;
use serde::{Deserialize, Serialize};
use crate::context::CommandContext;
use crate::output::{CommandOutput, EzError};

#[derive(Serialize, Deserialize)]
pub struct ExplainResult {
    pub command: String,
    pub args: Vec<String>,
    pub breakdown: Vec<BreakdownItem>,
    pub plain_english: String,
    pub stages: Option<Vec<StageResult>>,
}

#[derive(Serialize, Deserialize)]
pub struct BreakdownItem {
    pub part: String,
    pub meaning: String,
}

#[derive(Serialize, Deserialize)]
pub struct StageResult {
    pub stage: usize,
    pub command: String,
    pub breakdown: Vec<BreakdownItem>,
}

struct CommandInfo {
    #[allow(dead_code)]
    name: &'static str,
    description: &'static str,
    flags: HashMap<&'static str, &'static str>,
}

fn build_knowledge_base() -> HashMap<&'static str, CommandInfo> {
    let mut kb = HashMap::new();

    // find
    let mut find_flags = HashMap::new();
    find_flags.insert("-name", "Search by filename pattern");
    find_flags.insert("-type", "Search by file type (f=file, d=directory)");
    find_flags.insert("-size", "Search by file size");
    find_flags.insert("-exec", "Execute command on each found file");
    find_flags.insert("-mtime", "Modified within N days");
    find_flags.insert("-mmin", "Modified within N minutes");
    find_flags.insert("-maxdepth", "Limit search depth");
    find_flags.insert("-mindepth", "Minimum search depth");
    find_flags.insert("-print", "Print full path (default)");
    find_flags.insert("-printf", "Print formatted output");
    find_flags.insert("-delete", "Delete found files");
    find_flags.insert("-empty", "Find empty files/directories");
    find_flags.insert("-user", "Find files owned by user");
    find_flags.insert("-group", "Find files owned by group");
    find_flags.insert("-perm", "Find by permissions");
    find_flags.insert("-regex", "Search with regex pattern");
    find_flags.insert("-iregex", "Case-insensitive regex search");
    find_flags.insert("-iname", "Case-insensitive name search");
    find_flags.insert("-path", "Match full path pattern");
    find_flags.insert("-prune", "Skip descending into directory");
    find_flags.insert("-ls", "List files like ls -dils");
    find_flags.insert("-ok", "Like -exec but ask first");
    kb.insert("find", CommandInfo {
        name: "find",
        description: "Search for files and directories",
        flags: find_flags,
    });

    // grep
    let mut grep_flags = HashMap::new();
    grep_flags.insert("-i", "Case-insensitive search");
    grep_flags.insert("-v", "Invert match (show non-matching lines)");
    grep_flags.insert("-r", "Recursive search");
    grep_flags.insert("-l", "Show only filenames with matches");
    grep_flags.insert("-n", "Show line numbers");
    grep_flags.insert("-c", "Count matching lines");
    grep_flags.insert("-w", "Match whole words only");
    grep_flags.insert("-x", "Match whole lines only");
    grep_flags.insert("-E", "Extended regex patterns");
    grep_flags.insert("-F", "Fixed strings (no regex)");
    grep_flags.insert("-o", "Show only matching parts");
    grep_flags.insert("-A", "Show N lines after match");
    grep_flags.insert("-B", "Show N lines before match");
    grep_flags.insert("-C", "Show N lines around match");
    grep_flags.insert("-h", "Suppress filenames");
    grep_flags.insert("-H", "Always show filenames");
    grep_flags.insert("--color", "Highlight matches in color");
    kb.insert("grep", CommandInfo {
        name: "grep",
        description: "Search text using patterns",
        flags: grep_flags,
    });

    // awk
    let mut awk_flags = HashMap::new();
    awk_flags.insert("-F", "Set field separator");
    awk_flags.insert("-v", "Set variable value");
    awk_flags.insert("-f", "Read program from file");
    kb.insert("awk", CommandInfo {
        name: "awk",
        description: "Pattern scanning and text processing",
        flags: awk_flags,
    });

    // sed
    let mut sed_flags = HashMap::new();
    sed_flags.insert("-i", "Edit files in-place");
    sed_flags.insert("-n", "Suppress automatic printing");
    sed_flags.insert("-e", "Add script expression");
    sed_flags.insert("-f", "Read script from file");
    sed_flags.insert("-r", "Use extended regex");
    sed_flags.insert("-E", "Use extended regex");
    kb.insert("sed", CommandInfo {
        name: "sed",
        description: "Stream editor for filtering/transforming text",
        flags: sed_flags,
    });

    // tar
    let mut tar_flags = HashMap::new();
    tar_flags.insert("-c", "Create archive");
    tar_flags.insert("-x", "Extract from archive");
    tar_flags.insert("-t", "List contents");
    tar_flags.insert("-f", "Specify archive file");
    tar_flags.insert("-v", "Verbose output");
    tar_flags.insert("-z", "Compress with gzip");
    tar_flags.insert("-j", "Compress with bzip2");
    tar_flags.insert("-J", "Compress with xz");
    tar_flags.insert("-p", "Preserve permissions");
    tar_flags.insert("-C", "Change to directory");
    tar_flags.insert("--exclude", "Exclude files matching pattern");
    kb.insert("tar", CommandInfo {
        name: "tar",
        description: "Archive utility for files",
        flags: tar_flags,
    });

    // chmod
    let mut chmod_flags = HashMap::new();
    chmod_flags.insert("-R", "Recursive");
    chmod_flags.insert("-v", "Verbose");
    kb.insert("chmod", CommandInfo {
        name: "chmod",
        description: "Change file permissions",
        flags: chmod_flags,
    });

    // chown
    let mut chown_flags = HashMap::new();
    chown_flags.insert("-R", "Recursive");
    chown_flags.insert("-v", "Verbose");
    kb.insert("chown", CommandInfo {
        name: "chown",
        description: "Change file owner and group",
        flags: chown_flags,
    });

    // curl
    let mut curl_flags = HashMap::new();
    curl_flags.insert("-o", "Output to file");
    curl_flags.insert("-O", "Save with remote filename");
    curl_flags.insert("-L", "Follow redirects");
    curl_flags.insert("-I", "Show headers only");
    curl_flags.insert("-X", "Specify HTTP method");
    curl_flags.insert("-H", "Add header");
    curl_flags.insert("-d", "Send POST data");
    curl_flags.insert("-s", "Silent (no progress)");
    curl_flags.insert("-S", "Show errors");
    curl_flags.insert("-u", "User authentication");
    curl_flags.insert("-k", "Allow insecure SSL");
    curl_flags.insert("-v", "Verbose");
    kb.insert("curl", CommandInfo {
        name: "curl",
        description: "Transfer data from/to a URL",
        flags: curl_flags,
    });

    // wget
    let mut wget_flags = HashMap::new();
    wget_flags.insert("-O", "Output to file");
    wget_flags.insert("-P", "Save to directory");
    wget_flags.insert("-q", "Quiet mode");
    wget_flags.insert("-c", "Continue partial download");
    wget_flags.insert("-r", "Recursive download");
    wget_flags.insert("-l", "Maximum recursion depth");
    kb.insert("wget", CommandInfo {
        name: "wget",
        description: "Non-interactive network downloader",
        flags: wget_flags,
    });

    // ssh
    let mut ssh_flags = HashMap::new();
    ssh_flags.insert("-p", "Port number");
    ssh_flags.insert("-i", "Identity file (private key)");
    ssh_flags.insert("-X", "Enable X11 forwarding");
    ssh_flags.insert("-Y", "Trusted X11 forwarding");
    ssh_flags.insert("-v", "Verbose (debug)");
    ssh_flags.insert("-q", "Quiet mode");
    ssh_flags.insert("-N", "No remote commands");
    ssh_flags.insert("-f", "Background mode");
    ssh_flags.insert("-L", "Port forwarding");
    kb.insert("ssh", CommandInfo {
        name: "ssh",
        description: "Secure shell remote login",
        flags: ssh_flags,
    });

    // scp
    let mut scp_flags = HashMap::new();
    scp_flags.insert("-P", "Port number");
    scp_flags.insert("-i", "Identity file");
    scp_flags.insert("-r", "Recursive copy");
    scp_flags.insert("-p", "Preserve attributes");
    scp_flags.insert("-q", "Quiet mode");
    scp_flags.insert("-C", "Enable compression");
    kb.insert("scp", CommandInfo {
        name: "scp",
        description: "Secure copy (remote file copy)",
        flags: scp_flags,
    });

    // rsync
    let mut rsync_flags = HashMap::new();
    rsync_flags.insert("-a", "Archive mode (preserves everything)");
    rsync_flags.insert("-v", "Verbose");
    rsync_flags.insert("-z", "Compress during transfer");
    rsync_flags.insert("-P", "Show progress, allow resume");
    rsync_flags.insert("-r", "Recursive");
    rsync_flags.insert("--delete", "Delete dest files not in src");
    rsync_flags.insert("-n", "Dry run (simulate)");
    rsync_flags.insert("-e", "Specify remote shell");
    kb.insert("rsync", CommandInfo {
        name: "rsync",
        description: "Fast remote file sync utility",
        flags: rsync_flags,
    });

    // xargs
    let mut xargs_flags = HashMap::new();
    xargs_flags.insert("-n", "Max args per command");
    xargs_flags.insert("-I", "Replace string");
    xargs_flags.insert("-P", "Parallel processes");
    xargs_flags.insert("-0", "Input is null-separated");
    xargs_flags.insert("-t", "Print commands before executing");
    xargs_flags.insert("-p", "Prompt before running");
    kb.insert("xargs", CommandInfo {
        name: "xargs",
        description: "Build and execute commands from stdin",
        flags: xargs_flags,
    });

    // sort
    let mut sort_flags = HashMap::new();
    sort_flags.insert("-r", "Reverse order");
    sort_flags.insert("-n", "Numeric sort");
    sort_flags.insert("-k", "Sort by key/column");
    sort_flags.insert("-t", "Field separator");
    sort_flags.insert("-u", "Unique only (remove duplicates)");
    sort_flags.insert("-f", "Case-insensitive");
    sort_flags.insert("-M", "Month sort");
    sort_flags.insert("-h", "Human numeric sort");
    kb.insert("sort", CommandInfo {
        name: "sort",
        description: "Sort lines of text files",
        flags: sort_flags,
    });

    // uniq
    let mut uniq_flags = HashMap::new();
    uniq_flags.insert("-c", "Count occurrences");
    uniq_flags.insert("-d", "Show only duplicates");
    uniq_flags.insert("-D", "Show all duplicates");
    uniq_flags.insert("-u", "Show only unique lines");
    uniq_flags.insert("-i", "Case-insensitive");
    uniq_flags.insert("-f", "Skip first N fields");
    uniq_flags.insert("-s", "Skip first N characters");
    uniq_flags.insert("-w", "Compare only N characters");
    kb.insert("uniq", CommandInfo {
        name: "uniq",
        description: "Report or filter duplicate lines",
        flags: uniq_flags,
    });

    // cut
    let mut cut_flags = HashMap::new();
    cut_flags.insert("-d", "Delimiter");
    cut_flags.insert("-f", "Select fields");
    cut_flags.insert("-c", "Select characters");
    cut_flags.insert("-b", "Select bytes");
    cut_flags.insert("--complement", "Invert selection");
    kb.insert("cut", CommandInfo {
        name: "cut",
        description: "Remove sections from lines",
        flags: cut_flags,
    });

    // tr
    let mut tr_flags = HashMap::new();
    tr_flags.insert("-d", "Delete characters");
    tr_flags.insert("-s", "Squeeze repeats");
    tr_flags.insert("-c", "Complement set");
    kb.insert("tr", CommandInfo {
        name: "tr",
        description: "Translate/delete characters",
        flags: tr_flags,
    });

    // head
    let mut head_flags = HashMap::new();
    head_flags.insert("-n", "Number of lines");
    head_flags.insert("-c", "Number of bytes");
    head_flags.insert("-q", "Never print headers");
    head_flags.insert("-v", "Always print headers");
    kb.insert("head", CommandInfo {
        name: "head",
        description: "Output first part of files",
        flags: head_flags,
    });

    // tail
    let mut tail_flags = HashMap::new();
    tail_flags.insert("-n", "Number of lines");
    tail_flags.insert("-c", "Number of bytes");
    tail_flags.insert("-f", "Follow file changes");
    tail_flags.insert("-F", "Follow with retry");
    kb.insert("tail", CommandInfo {
        name: "tail",
        description: "Output last part of files",
        flags: tail_flags,
    });

    // tee
    let mut tee_flags = HashMap::new();
    tee_flags.insert("-a", "Append to file");
    tee_flags.insert("-i", "Ignore interrupts");
    kb.insert("tee", CommandInfo {
        name: "tee",
        description: "Read from stdin and write to file and stdout",
        flags: tee_flags,
    });

    // wc
    let mut wc_flags = HashMap::new();
    wc_flags.insert("-l", "Count lines");
    wc_flags.insert("-w", "Count words");
    wc_flags.insert("-c", "Count bytes");
    wc_flags.insert("-m", "Count characters");
    wc_flags.insert("-L", "Print max line length");
    kb.insert("wc", CommandInfo {
        name: "wc",
        description: "Count lines, words, and bytes",
        flags: wc_flags,
    });

    // diff
    let mut diff_flags = HashMap::new();
    diff_flags.insert("-u", "Unified diff format");
    diff_flags.insert("-c", "Context diff format");
    diff_flags.insert("-r", "Recursive");
    diff_flags.insert("-i", "Case-insensitive");
    diff_flags.insert("-w", "Ignore whitespace");
    diff_flags.insert("-B", "Ignore blank lines");
    kb.insert("diff", CommandInfo {
        name: "diff",
        description: "Compare files line by line",
        flags: diff_flags,
    });

    // ln
    let mut ln_flags = HashMap::new();
    ln_flags.insert("-s", "Symbolic link");
    ln_flags.insert("-f", "Force (overwrite)");
    ln_flags.insert("-i", "Interactive");
    ln_flags.insert("-v", "Verbose");
    kb.insert("ln", CommandInfo {
        name: "ln",
        description: "Make links between files",
        flags: ln_flags,
    });

    // df
    let mut df_flags = HashMap::new();
    df_flags.insert("-h", "Human-readable sizes");
    df_flags.insert("-T", "Show filesystem type");
    df_flags.insert("-i", "Show inode info");
    kb.insert("df", CommandInfo {
        name: "df",
        description: "Report disk space usage",
        flags: df_flags,
    });

    // du
    let mut du_flags = HashMap::new();
    du_flags.insert("-h", "Human-readable sizes");
    du_flags.insert("-s", "Summary only");
    du_flags.insert("-a", "Show all files");
    du_flags.insert("-c", "Show total");
    du_flags.insert("--max-depth", "Limit depth");
    kb.insert("du", CommandInfo {
        name: "du",
        description: "Estimate file space usage",
        flags: du_flags,
    });

    // ps
    let mut ps_flags = HashMap::new();
    ps_flags.insert("aux", "Show all processes (BSD style)");
    ps_flags.insert("-ef", "Show all processes (standard)");
    ps_flags.insert("-eo", "Custom output format");
    kb.insert("ps", CommandInfo {
        name: "ps",
        description: "Report process status",
        flags: ps_flags,
    });

    // kill
    let mut kill_flags = HashMap::new();
    kill_flags.insert("-9", "SIGKILL (force)");
    kill_flags.insert("-15", "SIGTERM (graceful)");
    kill_flags.insert("-HUP", "SIGHUP (reload)");
    kb.insert("kill", CommandInfo {
        name: "kill",
        description: "Send signal to process",
        flags: kill_flags,
    });

    // top
    kb.insert("top", CommandInfo {
        name: "top",
        description: "Display system processes",
        flags: HashMap::new(),
    });

    // netstat
    let mut netstat_flags = HashMap::new();
    netstat_flags.insert("-t", "TCP sockets");
    netstat_flags.insert("-u", "UDP sockets");
    netstat_flags.insert("-l", "Listening sockets");
    netstat_flags.insert("-n", "Numeric output");
    netstat_flags.insert("-p", "Show PID/program");
    netstat_flags.insert("-a", "All sockets");
    kb.insert("netstat", CommandInfo {
        name: "netstat",
        description: "Network statistics",
        flags: netstat_flags,
    });

    // ip
    let mut ip_flags = HashMap::new();
    ip_flags.insert("addr", "Show addresses");
    ip_flags.insert("link", "Show interfaces");
    ip_flags.insert("route", "Show routing table");
    kb.insert("ip", CommandInfo {
        name: "ip",
        description: "Show/manipulate routing, devices, and tunnels",
        flags: ip_flags,
    });

    // iptables
    let mut iptables_flags = HashMap::new();
    iptables_flags.insert("-A", "Append rule");
    iptables_flags.insert("-D", "Delete rule");
    iptables_flags.insert("-L", "List rules");
    iptables_flags.insert("-F", "Flush rules");
    iptables_flags.insert("-I", "Insert rule");
    iptables_flags.insert("-p", "Protocol");
    iptables_flags.insert("-j", "Jump target");
    kb.insert("iptables", CommandInfo {
        name: "iptables",
        description: "IP packet filter administration",
        flags: iptables_flags,
    });

    // crontab
    let mut crontab_flags = HashMap::new();
    crontab_flags.insert("-l", "List crontab");
    crontab_flags.insert("-e", "Edit crontab");
    crontab_flags.insert("-r", "Remove crontab");
    kb.insert("crontab", CommandInfo {
        name: "crontab",
        description: "Maintain crontab files",
        flags: crontab_flags,
    });

    // docker
    let mut docker_flags = HashMap::new();
    docker_flags.insert("ps", "List containers");
    docker_flags.insert("run", "Run container");
    docker_flags.insert("exec", "Execute in container");
    docker_flags.insert("build", "Build image");
    docker_flags.insert("images", "List images");
    docker_flags.insert("pull", "Pull image");
    docker_flags.insert("push", "Push image");
    docker_flags.insert("rm", "Remove container");
    docker_flags.insert("rmi", "Remove image");
    docker_flags.insert("-d", "Detached mode");
    docker_flags.insert("-it", "Interactive TTY");
    docker_flags.insert("-p", "Port mapping");
    docker_flags.insert("-v", "Volume mount");
    kb.insert("docker", CommandInfo {
        name: "docker",
        description: "Container platform",
        flags: docker_flags,
    });

    // git
    let mut git_flags = HashMap::new();
    git_flags.insert("add", "Add files to staging");
    git_flags.insert("commit", "Record changes");
    git_flags.insert("push", "Upload to remote");
    git_flags.insert("pull", "Download from remote");
    git_flags.insert("clone", "Copy repository");
    git_flags.insert("status", "Show working tree status");
    git_flags.insert("log", "Show commit history");
    git_flags.insert("branch", "List/create branches");
    git_flags.insert("checkout", "Switch branches");
    git_flags.insert("merge", "Join branches");
    git_flags.insert("diff", "Show changes");
    git_flags.insert("reset", "Reset state");
    git_flags.insert("-m", "Commit message");
    git_flags.insert("-a", "All files");
    git_flags.insert("-b", "Create branch");
    kb.insert("git", CommandInfo {
        name: "git",
        description: "Distributed version control",
        flags: git_flags,
    });

    // ffmpeg
    let mut ffmpeg_flags = HashMap::new();
    ffmpeg_flags.insert("-i", "Input file");
    ffmpeg_flags.insert("-c:v", "Video codec");
    ffmpeg_flags.insert("-c:a", "Audio codec");
    ffmpeg_flags.insert("-crf", "Constant rate factor");
    ffmpeg_flags.insert("-b:v", "Video bitrate");
    ffmpeg_flags.insert("-r", "Frame rate");
    ffmpeg_flags.insert("-s", "Resolution");
    ffmpeg_flags.insert("-ss", "Start time");
    ffmpeg_flags.insert("-t", "Duration");
    ffmpeg_flags.insert("-vn", "No video");
    ffmpeg_flags.insert("-an", "No audio");
    kb.insert("ffmpeg", CommandInfo {
        name: "ffmpeg",
        description: "Audio/video converter",
        flags: ffmpeg_flags,
    });

    // make
    let mut make_flags = HashMap::new();
    make_flags.insert("-j", "Parallel jobs");
    make_flags.insert("-f", "Specify makefile");
    make_flags.insert("-n", "Dry run");
    make_flags.insert("-B", "Unconditionally make");
    kb.insert("make", CommandInfo {
        name: "make",
        description: "Build automation tool",
        flags: make_flags,
    });

    // gcc
    let mut gcc_flags = HashMap::new();
    gcc_flags.insert("-o", "Output file");
    gcc_flags.insert("-c", "Compile only");
    gcc_flags.insert("-g", "Debug info");
    gcc_flags.insert("-O", "Optimization level");
    gcc_flags.insert("-Wall", "All warnings");
    gcc_flags.insert("-I", "Include path");
    gcc_flags.insert("-L", "Library path");
    gcc_flags.insert("-l", "Link library");
    gcc_flags.insert("-static", "Static linking");
    gcc_flags.insert("-shared", "Shared library");
    kb.insert("gcc", CommandInfo {
        name: "gcc",
        description: "GNU C compiler",
        flags: gcc_flags,
    });

    // ls
    let mut ls_flags = HashMap::new();
    ls_flags.insert("-l", "Long format with details");
    ls_flags.insert("-a", "Show hidden files");
    ls_flags.insert("-h", "Human-readable sizes");
    ls_flags.insert("-t", "Sort by time");
    ls_flags.insert("-r", "Reverse order");
    ls_flags.insert("-S", "Sort by size");
    ls_flags.insert("-R", "Recursive");
    ls_flags.insert("-d", "List directories");
    ls_flags.insert("-i", "Show inode");
    ls_flags.insert("-F", "Add type indicator");
    kb.insert("ls", CommandInfo {
        name: "ls",
        description: "List directory contents",
        flags: ls_flags,
    });

    // cat
    let mut cat_flags = HashMap::new();
    cat_flags.insert("-n", "Number lines");
    cat_flags.insert("-b", "Number non-blank lines");
    cat_flags.insert("-E", "Show line endings");
    cat_flags.insert("-T", "Show tabs");
    cat_flags.insert("-A", "Show all special chars");
    kb.insert("cat", CommandInfo {
        name: "cat",
        description: "Concatenate and print files",
        flags: cat_flags,
    });

    // cd (shell builtin but commonly used)
    kb.insert("cd", CommandInfo {
        name: "cd",
        description: "Change directory",
        flags: HashMap::new(),
    });

    // pwd
    kb.insert("pwd", CommandInfo {
        name: "pwd",
        description: "Print working directory",
        flags: HashMap::new(),
    });

    // echo
    let mut echo_flags = HashMap::new();
    echo_flags.insert("-n", "No trailing newline");
    echo_flags.insert("-e", "Enable escape sequences");
    kb.insert("echo", CommandInfo {
        name: "echo",
        description: "Print text to stdout",
        flags: echo_flags,
    });

    // mkdir
    let mut mkdir_flags = HashMap::new();
    mkdir_flags.insert("-p", "Create parent directories");
    mkdir_flags.insert("-v", "Verbose");
    kb.insert("mkdir", CommandInfo {
        name: "mkdir",
        description: "Create directories",
        flags: mkdir_flags,
    });

    // rm
    let mut rm_flags = HashMap::new();
    rm_flags.insert("-r", "Recursive");
    rm_flags.insert("-f", "Force (no prompt)");
    rm_flags.insert("-i", "Interactive");
    rm_flags.insert("-v", "Verbose");
    kb.insert("rm", CommandInfo {
        name: "rm",
        description: "Remove files/directories",
        flags: rm_flags,
    });

    // cp
    let mut cp_flags = HashMap::new();
    cp_flags.insert("-r", "Recursive");
    cp_flags.insert("-p", "Preserve attributes");
    cp_flags.insert("-v", "Verbose");
    cp_flags.insert("-f", "Force");
    cp_flags.insert("-i", "Interactive");
    kb.insert("cp", CommandInfo {
        name: "cp",
        description: "Copy files/directories",
        flags: cp_flags,
    });

    // mv
    let mut mv_flags = HashMap::new();
    mv_flags.insert("-f", "Force");
    mv_flags.insert("-i", "Interactive");
    mv_flags.insert("-v", "Verbose");
    mv_flags.insert("-n", "No clobber");
    kb.insert("mv", CommandInfo {
        name: "mv",
        description: "Move/rename files",
        flags: mv_flags,
    });

    // touch
    kb.insert("touch", CommandInfo {
        name: "touch",
        description: "Create empty file or update timestamp",
        flags: HashMap::new(),
    });

    // less
    let mut less_flags = HashMap::new();
    less_flags.insert("-N", "Show line numbers");
    less_flags.insert("-i", "Case-insensitive search");
    less_flags.insert("+F", "Follow file like tail");
    kb.insert("less", CommandInfo {
        name: "less",
        description: "Pager for viewing text",
        flags: less_flags,
    });

    // more
    kb.insert("more", CommandInfo {
        name: "more",
        description: "Pager for viewing text (simple)",
        flags: HashMap::new(),
    });

    // man
    let mut man_flags = HashMap::new();
    man_flags.insert("-k", "Search for keyword");
    man_flags.insert("-f", "Show short description");
    kb.insert("man", CommandInfo {
        name: "man",
        description: "Display manual pages",
        flags: man_flags,
    });

    // which
    kb.insert("which", CommandInfo {
        name: "which",
        description: "Locate command in PATH",
        flags: HashMap::new(),
    });

    // whereis
    kb.insert("whereis", CommandInfo {
        name: "whereis",
        description: "Locate binary/source/manual",
        flags: HashMap::new(),
    });

    // whoami
    kb.insert("whoami", CommandInfo {
        name: "whoami",
        description: "Print current user",
        flags: HashMap::new(),
    });

    // id
    kb.insert("id", CommandInfo {
        name: "id",
        description: "Print user/group ID",
        flags: HashMap::new(),
    });

    // groups
    kb.insert("groups", CommandInfo {
        name: "groups",
        description: "Print group membership",
        flags: HashMap::new(),
    });

    // uname
    let mut uname_flags = HashMap::new();
    uname_flags.insert("-a", "All information");
    uname_flags.insert("-r", "Kernel release");
    uname_flags.insert("-m", "Machine hardware");
    kb.insert("uname", CommandInfo {
        name: "uname",
        description: "Print system information",
        flags: uname_flags,
    });

    // date
    let mut date_flags = HashMap::new();
    date_flags.insert("+", "Format string");
    date_flags.insert("-u", "UTC time");
    kb.insert("date", CommandInfo {
        name: "date",
        description: "Print/set system date",
        flags: date_flags,
    });

    // cal
    kb.insert("cal", CommandInfo {
        name: "cal",
        description: "Display calendar",
        flags: HashMap::new(),
    });

    // clear
    kb.insert("clear", CommandInfo {
        name: "clear",
        description: "Clear terminal screen",
        flags: HashMap::new(),
    });

    // history
    kb.insert("history", CommandInfo {
        name: "history",
        description: "Show command history",
        flags: HashMap::new(),
    });

    // alias
    kb.insert("alias", CommandInfo {
        name: "alias",
        description: "Create command alias",
        flags: HashMap::new(),
    });

    // source
    kb.insert("source", CommandInfo {
        name: "source",
        description: "Execute commands from file",
        flags: HashMap::new(),
    });

    // export
    kb.insert("export", CommandInfo {
        name: "export",
        description: "Set environment variable",
        flags: HashMap::new(),
    });

    // env
    kb.insert("env", CommandInfo {
        name: "env",
        description: "Run program in modified environment",
        flags: HashMap::new(),
    });

    // printenv
    kb.insert("printenv", CommandInfo {
        name: "printenv",
        description: "Print environment variables",
        flags: HashMap::new(),
    });

    // uptime
    kb.insert("uptime", CommandInfo {
        name: "uptime",
        description: "Show system uptime",
        flags: HashMap::new(),
    });

    // free
    let mut free_flags = HashMap::new();
    free_flags.insert("-h", "Human-readable");
    free_flags.insert("-m", "Show in MB");
    free_flags.insert("-g", "Show in GB");
    kb.insert("free", CommandInfo {
        name: "free",
        description: "Show memory usage",
        flags: free_flags,
    });

    // mount
    kb.insert("mount", CommandInfo {
        name: "mount",
        description: "Mount filesystem",
        flags: HashMap::new(),
    });

    // umount
    kb.insert("umount", CommandInfo {
        name: "umount",
        description: "Unmount filesystem",
        flags: HashMap::new(),
    });

    // ping
    let mut ping_flags = HashMap::new();
    ping_flags.insert("-c", "Count packets");
    ping_flags.insert("-i", "Interval");
    ping_flags.insert("-s", "Packet size");
    ping_flags.insert("-W", "Timeout");
    kb.insert("ping", CommandInfo {
        name: "ping",
        description: "Test network connectivity",
        flags: ping_flags,
    });

    // traceroute
    kb.insert("traceroute", CommandInfo {
        name: "traceroute",
        description: "Trace network route",
        flags: HashMap::new(),
    });

    // dig
    kb.insert("dig", CommandInfo {
        name: "dig",
        description: "DNS lookup utility",
        flags: HashMap::new(),
    });

    // nslookup
    kb.insert("nslookup", CommandInfo {
        name: "nslookup",
        description: "Query DNS servers",
        flags: HashMap::new(),
    });

    // hostname
    kb.insert("hostname", CommandInfo {
        name: "hostname",
        description: "Show/set hostname",
        flags: HashMap::new(),
    });

    // ifconfig
    kb.insert("ifconfig", CommandInfo {
        name: "ifconfig",
        description: "Configure network interfaces",
        flags: HashMap::new(),
    });

    // zip
    let mut zip_flags = HashMap::new();
    zip_flags.insert("-r", "Recursive");
    zip_flags.insert("-q", "Quiet");
    zip_flags.insert("-e", "Encrypt");
    kb.insert("zip", CommandInfo {
        name: "zip",
        description: "Package and compress files",
        flags: zip_flags,
    });

    // unzip
    let mut unzip_flags = HashMap::new();
    unzip_flags.insert("-l", "List contents");
    unzip_flags.insert("-d", "Destination directory");
    unzip_flags.insert("-o", "Overwrite");
    unzip_flags.insert("-q", "Quiet");
    kb.insert("unzip", CommandInfo {
        name: "unzip",
        description: "Extract zip archive",
        flags: unzip_flags,
    });

    // gzip
    let mut gzip_flags = HashMap::new();
    gzip_flags.insert("-d", "Decompress");
    gzip_flags.insert("-k", "Keep original");
    gzip_flags.insert("-v", "Verbose");
    kb.insert("gzip", CommandInfo {
        name: "gzip",
        description: "Compress files",
        flags: gzip_flags,
    });

    // gunzip
    kb.insert("gunzip", CommandInfo {
        name: "gunzip",
        description: "Decompress gzip files",
        flags: HashMap::new(),
    });

    // bzip2
    kb.insert("bzip2", CommandInfo {
        name: "bzip2",
        description: "Block-sorting file compressor",
        flags: HashMap::new(),
    });

    // bunzip2
    kb.insert("bunzip2", CommandInfo {
        name: "bunzip2",
        description: "Decompress bzip2 files",
        flags: HashMap::new(),
    });

    // xz
    kb.insert("xz", CommandInfo {
        name: "xz",
        description: "XZ compression utility",
        flags: HashMap::new(),
    });

    // nano
    kb.insert("nano", CommandInfo {
        name: "nano",
        description: "Simple text editor",
        flags: HashMap::new(),
    });

    // vim/vi
    kb.insert("vim", CommandInfo {
        name: "vim",
        description: "Vi IMproved text editor",
        flags: HashMap::new(),
    });
    kb.insert("vi", CommandInfo {
        name: "vi",
        description: "Visual text editor",
        flags: HashMap::new(),
    });

    // emacs
    kb.insert("emacs", CommandInfo {
        name: "emacs",
        description: "GNU text editor",
        flags: HashMap::new(),
    });

    // screen
    let mut screen_flags = HashMap::new();
    screen_flags.insert("-S", "Session name");
    screen_flags.insert("-ls", "List sessions");
    screen_flags.insert("-r", "Reattach");
    screen_flags.insert("-d", "Detach");
    kb.insert("screen", CommandInfo {
        name: "screen",
        description: "Terminal multiplexer",
        flags: screen_flags,
    });

    // tmux
    let mut tmux_flags = HashMap::new();
    tmux_flags.insert("new", "New session");
    tmux_flags.insert("attach", "Attach to session");
    tmux_flags.insert("ls", "List sessions");
    tmux_flags.insert("kill-session", "Kill session");
    kb.insert("tmux", CommandInfo {
        name: "tmux",
        description: "Terminal multiplexer",
        flags: tmux_flags,
    });

    // nohup
    kb.insert("nohup", CommandInfo {
        name: "nohup",
        description: "Run command immune to hangups",
        flags: HashMap::new(),
    });

    // bg
    kb.insert("bg", CommandInfo {
        name: "bg",
        description: "Resume job in background",
        flags: HashMap::new(),
    });

    // fg
    kb.insert("fg", CommandInfo {
        name: "fg",
        description: "Resume job in foreground",
        flags: HashMap::new(),
    });

    // jobs
    kb.insert("jobs", CommandInfo {
        name: "jobs",
        description: "List active jobs",
        flags: HashMap::new(),
    });

    // exit
    kb.insert("exit", CommandInfo {
        name: "exit",
        description: "Exit the shell",
        flags: HashMap::new(),
    });

    // logout
    kb.insert("logout", CommandInfo {
        name: "logout",
        description: "Logout from shell",
        flags: HashMap::new(),
    });

    // su
    kb.insert("su", CommandInfo {
        name: "su",
        description: "Switch user",
        flags: HashMap::new(),
    });

    // sudo
    let mut sudo_flags = HashMap::new();
    sudo_flags.insert("-u", "Run as user");
    sudo_flags.insert("-i", "Login shell");
    sudo_flags.insert("-E", "Preserve environment");
    sudo_flags.insert("-H", "Set HOME");
    kb.insert("sudo", CommandInfo {
        name: "sudo",
        description: "Execute as superuser",
        flags: sudo_flags,
    });

    // passwd
    kb.insert("passwd", CommandInfo {
        name: "passwd",
        description: "Change password",
        flags: HashMap::new(),
    });

    // adduser/useradd
    kb.insert("adduser", CommandInfo {
        name: "adduser",
        description: "Add a user",
        flags: HashMap::new(),
    });

    // deluser/userdel
    kb.insert("deluser", CommandInfo {
        name: "deluser",
        description: "Delete a user",
        flags: HashMap::new(),
    });

    // apt/apt-get
    let mut apt_flags = HashMap::new();
    apt_flags.insert("install", "Install package");
    apt_flags.insert("remove", "Remove package");
    apt_flags.insert("update", "Update package list");
    apt_flags.insert("upgrade", "Upgrade packages");
    apt_flags.insert("search", "Search for package");
    apt_flags.insert("show", "Show package info");
    apt_flags.insert("list", "List packages");
    apt_flags.insert("autoremove", "Remove unused");
    apt_flags.insert("purge", "Remove with config");
    kb.insert("apt", CommandInfo {
        name: "apt",
        description: "Package manager (Debian/Ubuntu)",
        flags: apt_flags.clone(),
    });
    kb.insert("apt-get", CommandInfo {
        name: "apt-get",
        description: "Package manager (lower-level)",
        flags: apt_flags,
    });

    // yum/dnf
    kb.insert("yum", CommandInfo {
        name: "yum",
        description: "Package manager (RHEL/CentOS)",
        flags: HashMap::new(),
    });
    kb.insert("dnf", CommandInfo {
        name: "dnf",
        description: "Next-gen package manager (Fedora)",
        flags: HashMap::new(),
    });

    // pacman
    kb.insert("pacman", CommandInfo {
        name: "pacman",
        description: "Package manager (Arch)",
        flags: HashMap::new(),
    });

    // snap
    kb.insert("snap", CommandInfo {
        name: "snap",
        description: "Snap package manager",
        flags: HashMap::new(),
    });

    // flatpak
    kb.insert("flatpak", CommandInfo {
        name: "flatpak",
        description: "Flatpak application manager",
        flags: HashMap::new(),
    });

    // systemctl
    let mut systemctl_flags = HashMap::new();
    systemctl_flags.insert("start", "Start service");
    systemctl_flags.insert("stop", "Stop service");
    systemctl_flags.insert("restart", "Restart service");
    systemctl_flags.insert("status", "Show service status");
    systemctl_flags.insert("enable", "Enable at boot");
    systemctl_flags.insert("disable", "Disable at boot");
    systemctl_flags.insert("list-units", "List units");
    kb.insert("systemctl", CommandInfo {
        name: "systemctl",
        description: "Control systemd services",
        flags: systemctl_flags,
    });

    // service
    kb.insert("service", CommandInfo {
        name: "service",
        description: "Run system service script",
        flags: HashMap::new(),
    });

    // journalctl
    let mut journalctl_flags = HashMap::new();
    journalctl_flags.insert("-u", "Show unit logs");
    journalctl_flags.insert("-f", "Follow");
    journalctl_flags.insert("-n", "Show last N lines");
    journalctl_flags.insert("--since", "Since time");
    journalctl_flags.insert("--until", "Until time");
    kb.insert("journalctl", CommandInfo {
        name: "journalctl",
        description: "View systemd logs",
        flags: journalctl_flags,
    });

    // dmesg
    kb.insert("dmesg", CommandInfo {
        name: "dmesg",
        description: "Print kernel messages",
        flags: HashMap::new(),
    });

    // lsb_release
    kb.insert("lsb_release", CommandInfo {
        name: "lsb_release",
        description: "Print distribution info",
        flags: HashMap::new(),
    });

    // lscpu
    kb.insert("lscpu", CommandInfo {
        name: "lscpu",
        description: "Display CPU info",
        flags: HashMap::new(),
    });

    // lsmem
    kb.insert("lsmem", CommandInfo {
        name: "lsmem",
        description: "List memory information",
        flags: HashMap::new(),
    });

    // lsblk
    kb.insert("lsblk", CommandInfo {
        name: "lsblk",
        description: "List block devices",
        flags: HashMap::new(),
    });

    // lspci
    kb.insert("lspci", CommandInfo {
        name: "lspci",
        description: "List PCI devices",
        flags: HashMap::new(),
    });

    // lsusb
    kb.insert("lsusb", CommandInfo {
        name: "lsusb",
        description: "List USB devices",
        flags: HashMap::new(),
    });

    // fdisk
    kb.insert("fdisk", CommandInfo {
        name: "fdisk",
        description: "Partition table manipulator",
        flags: HashMap::new(),
    });

    // mkfs
    kb.insert("mkfs", CommandInfo {
        name: "mkfs",
        description: "Build filesystem",
        flags: HashMap::new(),
    });

    // fsck
    kb.insert("fsck", CommandInfo {
        name: "fsck",
        description: "Filesystem check",
        flags: HashMap::new(),
    });

    // dd
    let mut dd_flags = HashMap::new();
    dd_flags.insert("if=", "Input file");
    dd_flags.insert("of=", "Output file");
    dd_flags.insert("bs=", "Block size");
    dd_flags.insert("count=", "Number of blocks");
    dd_flags.insert("status=", "Status output");
    kb.insert("dd", CommandInfo {
        name: "dd",
        description: "Convert and copy file",
        flags: dd_flags,
    });

    // stat
    kb.insert("stat", CommandInfo {
        name: "stat",
        description: "Display file status",
        flags: HashMap::new(),
    });

    // basename
    kb.insert("basename", CommandInfo {
        name: "basename",
        description: "Strip directory and suffix",
        flags: HashMap::new(),
    });

    // dirname
    kb.insert("dirname", CommandInfo {
        name: "dirname",
        description: "Strip filename from path",
        flags: HashMap::new(),
    });

    // realpath
    kb.insert("realpath", CommandInfo {
        name: "realpath",
        description: "Print canonical path",
        flags: HashMap::new(),
    });

    // readlink
    let mut readlink_flags = HashMap::new();
    readlink_flags.insert("-f", "Canonicalize");
    kb.insert("readlink", CommandInfo {
        name: "readlink",
        description: "Print symlink target",
        flags: readlink_flags,
    });

    // rev
    kb.insert("rev", CommandInfo {
        name: "rev",
        description: "Reverse lines character-wise",
        flags: HashMap::new(),
    });

    // tac
    kb.insert("tac", CommandInfo {
        name: "tac",
        description: "Concatenate and print reverse",
        flags: HashMap::new(),
    });

    // nl
    kb.insert("nl", CommandInfo {
        name: "nl",
        description: "Number lines of files",
        flags: HashMap::new(),
    });

    // paste
    kb.insert("paste", CommandInfo {
        name: "paste",
        description: "Merge lines of files",
        flags: HashMap::new(),
    });

    // join
    kb.insert("join", CommandInfo {
        name: "join",
        description: "Join lines on common field",
        flags: HashMap::new(),
    });

    // split
    kb.insert("split", CommandInfo {
        name: "split",
        description: "Split file into pieces",
        flags: HashMap::new(),
    });

    // csplit
    kb.insert("csplit", CommandInfo {
        name: "csplit",
        description: "Split by context",
        flags: HashMap::new(),
    });

    // comm
    kb.insert("comm", CommandInfo {
        name: "comm",
        description: "Compare sorted files",
        flags: HashMap::new(),
    });

    // shuf
    kb.insert("shuf", CommandInfo {
        name: "shuf",
        description: "Generate random permutations",
        flags: HashMap::new(),
    });

    // seq
    kb.insert("seq", CommandInfo {
        name: "seq",
        description: "Print sequence of numbers",
        flags: HashMap::new(),
    });

    // factor
    kb.insert("factor", CommandInfo {
        name: "factor",
        description: "Print prime factors",
        flags: HashMap::new(),
    });

    // expr
    kb.insert("expr", CommandInfo {
        name: "expr",
        description: "Evaluate expressions",
        flags: HashMap::new(),
    });

    // bc
    kb.insert("bc", CommandInfo {
        name: "bc",
        description: "Arbitrary precision calculator",
        flags: HashMap::new(),
    });

    // wc is already defined

    // timeout
    let mut timeout_flags = HashMap::new();
    timeout_flags.insert("-k", "Kill after");
    timeout_flags.insert("-s", "Signal to send");
    kb.insert("timeout", CommandInfo {
        name: "timeout",
        description: "Run with time limit",
        flags: timeout_flags,
    });

    // nice
    kb.insert("nice", CommandInfo {
        name: "nice",
        description: "Run with modified priority",
        flags: HashMap::new(),
    });

    // renice
    kb.insert("renice", CommandInfo {
        name: "renice",
        description: "Alter process priority",
        flags: HashMap::new(),
    });

    // wait
    kb.insert("wait", CommandInfo {
        name: "wait",
        description: "Wait for process completion",
        flags: HashMap::new(),
    });

    // pgrep
    kb.insert("pgrep", CommandInfo {
        name: "pgrep",
        description: "Find processes by name",
        flags: HashMap::new(),
    });

    // pkill
    kb.insert("pkill", CommandInfo {
        name: "pkill",
        description: "Signal processes by name",
        flags: HashMap::new(),
    });

    // pidof
    kb.insert("pidof", CommandInfo {
        name: "pidof",
        description: "Find PID of program",
        flags: HashMap::new(),
    });

    // fuser
    kb.insert("fuser", CommandInfo {
        name: "fuser",
        description: "Find processes using file",
        flags: HashMap::new(),
    });

    // lsof
    kb.insert("lsof", CommandInfo {
        name: "lsof",
        description: "List open files",
        flags: HashMap::new(),
    });

    // strace
    kb.insert("strace", CommandInfo {
        name: "strace",
        description: "Trace system calls",
        flags: HashMap::new(),
    });

    // ltrace
    kb.insert("ltrace", CommandInfo {
        name: "ltrace",
        description: "Trace library calls",
        flags: HashMap::new(),
    });

    // time
    kb.insert("time", CommandInfo {
        name: "time",
        description: "Time command execution",
        flags: HashMap::new(),
    });

    // watch
    let mut watch_flags = HashMap::new();
    watch_flags.insert("-n", "Interval seconds");
    watch_flags.insert("-d", "Highlight differences");
    kb.insert("watch", CommandInfo {
        name: "watch",
        description: "Execute periodically",
        flags: watch_flags,
    });

    // at
    kb.insert("at", CommandInfo {
        name: "at",
        description: "Queue command for later",
        flags: HashMap::new(),
    });

    // batch
    kb.insert("batch", CommandInfo {
        name: "batch",
        description: "Queue when load low",
        flags: HashMap::new(),
    });

    // atq
    kb.insert("atq", CommandInfo {
        name: "atq",
        description: "List at jobs",
        flags: HashMap::new(),
    });

    // atrm
    kb.insert("atrm", CommandInfo {
        name: "atrm",
        description: "Remove at jobs",
        flags: HashMap::new(),
    });

    // logger
    kb.insert("logger", CommandInfo {
        name: "logger",
        description: "Send message to syslog",
        flags: HashMap::new(),
    });

    kb
}

fn parse_command(command_str: &str) -> Vec<(String, Vec<String>)> {
    // Split by pipes first
    let stages: Vec<&str> = command_str.split('|').map(|s| s.trim()).collect();
    
    let mut result = Vec::new();
    
    for stage in stages {
        // Try to parse with shell-words
        match shell_words::split(stage) {
            Ok(parts) if !parts.is_empty() => {
                let cmd = parts[0].clone();
                let args = parts[1..].to_vec();
                result.push((cmd, args));
            }
            _ => {
                // Fallback: simple split
                let parts: Vec<&str> = stage.split_whitespace().collect();
                if !parts.is_empty() {
                    result.push((
                        parts[0].to_string(),
                        parts[1..].iter().map(|s| s.to_string()).collect(),
                    ));
                }
            }
        }
    }
    
    result
}

fn explain_single_command(cmd: &str, args: &[String], kb: &HashMap<&str, CommandInfo>) -> (Vec<BreakdownItem>, String) {
    let mut breakdown = Vec::new();
    let mut plain_parts = Vec::new();
    
    // Get command info
    if let Some(info) = kb.get(cmd) {
        breakdown.push(BreakdownItem {
            part: cmd.to_string(),
            meaning: info.description.to_string(),
        });
        plain_parts.push(info.description.to_string());
    } else {
        breakdown.push(BreakdownItem {
            part: cmd.to_string(),
            meaning: "Command".to_string(),
        });
        plain_parts.push(format!("Run '{}'", cmd));
    }
    
    // Explain arguments
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        
        // Check for flags
        if arg.starts_with('-') || arg.starts_with('+') {
            let flag_desc = if let Some(info) = kb.get(cmd) {
                info.flags.get(arg.as_str()).copied()
            } else {
                None
            };
            
            if let Some(desc) = flag_desc {
                breakdown.push(BreakdownItem {
                    part: arg.clone(),
                    meaning: desc.to_string(),
                });
                plain_parts.push(desc.to_string());
            } else if arg.starts_with("--") {
                // Long flag
                breakdown.push(BreakdownItem {
                    part: arg.clone(),
                    meaning: "Long option".to_string(),
                });
            } else {
                // Short flag not in our database
                breakdown.push(BreakdownItem {
                    part: arg.clone(),
                    meaning: "Option".to_string(),
                });
            }
            
            // Check if next arg is a value for this flag
            if i + 1 < args.len() && !args[i + 1].starts_with('-') {
                let value = &args[i + 1];
                
                // Special handling for certain patterns
                let meaning = if arg == "-name" || arg == "-iname" {
                    format!("Search pattern: {}", value)
                } else if arg == "-exec" {
                    "Command to execute".to_string()
                } else if arg == "-C" {
                    format!("Change to directory: {}", value)
                } else if arg == "-size" {
                    format!("File size: {}", value)
                } else if arg == "-type" {
                    let type_desc = match value.as_str() {
                        "f" => "files only",
                        "d" => "directories only",
                        "l" => "symlinks only",
                        _ => "type filter",
                    };
                    format!("{}", type_desc)
                } else if arg == "-mtime" || arg == "-mmin" {
                    format!("Modified within: {}", value)
                } else if arg == "-maxdepth" || arg == "-mindepth" {
                    format!("Depth: {}", value)
                } else {
                    format!("Value: {}", value)
                };
                
                breakdown.push(BreakdownItem {
                    part: value.clone(),
                    meaning,
                });
                i += 1;
            }
        } else if arg == "{}" {
            breakdown.push(BreakdownItem {
                part: arg.clone(),
                meaning: "Placeholder for found file".to_string(),
            });
        } else if arg == "+" || arg == "\\;" || arg == ";" {
            breakdown.push(BreakdownItem {
                part: arg.clone(),
                meaning: "End of -exec command (batch mode)".to_string(),
            });
        } else {
            // Regular argument (likely a filename)
            breakdown.push(BreakdownItem {
                part: arg.clone(),
                meaning: if arg.contains('.') { "File/directory".to_string() } else { "Argument".to_string() },
            });
        }
        
        i += 1;
    }
    
    let plain_english = generate_plain_english(cmd, args, &plain_parts);
    
    (breakdown, plain_english)
}

fn generate_plain_english(cmd: &str, args: &[String], _parts: &[String]) -> String {
    let mut plain = String::new();
    
    match cmd {
        "find" => {
            plain.push_str("Search for ");
            
            // Check for type
            let type_idx = args.iter().position(|a| a == "-type");
            if let Some(idx) = type_idx {
                if idx + 1 < args.len() {
                    match args[idx + 1].as_str() {
                        "f" => plain.push_str("files "),
                        "d" => plain.push_str("directories "),
                        "l" => plain.push_str("symlinks "),
                        _ => plain.push_str("items "),
                    }
                }
            } else {
                plain.push_str("files and directories ");
            }
            
            // Check for name pattern
            let name_idx = args.iter().position(|a| a == "-name" || a == "-iname");
            if let Some(idx) = name_idx {
                if idx + 1 < args.len() {
                    plain.push_str(&format!("matching '{}' ", args[idx + 1]));
                }
            }
            
            // Check for size
            let size_idx = args.iter().position(|a| a == "-size");
            if let Some(idx) = size_idx {
                if idx + 1 < args.len() {
                    plain.push_str(&format!("with size {} ", args[idx + 1]));
                }
            }
            
            // Check for exec
            if args.iter().any(|a| a == "-exec") {
                plain.push_str("and run a command on each result");
            }
        }
        "grep" => {
            let pattern_idx = args.iter().position(|a| !a.starts_with('-'));
            if let Some(idx) = pattern_idx {
                plain.push_str(&format!("Search for '{}' in text", args[idx]));
            } else {
                plain.push_str("Search for a pattern in text");
            }
            
            if args.iter().any(|a| a == "-i" || a == "--ignore-case") {
                plain.push_str(" (case-insensitive)");
            }
            if args.iter().any(|a| a == "-r" || a == "-R" || a == "--recursive") {
                plain.push_str(" recursively");
            }
            if args.iter().any(|a| a == "-l" || a == "--files-with-matches") {
                plain.push_str(", showing only filenames");
            }
        }
        "tar" => {
            if args.iter().any(|a| a == "-c" || a == "--create") {
                plain.push_str("Create an archive");
            } else if args.iter().any(|a| a == "-x" || a == "--extract") {
                plain.push_str("Extract files from an archive");
            } else if args.iter().any(|a| a == "-t" || a == "--list") {
                plain.push_str("List archive contents");
            } else {
                plain.push_str("Work with tar archives");
            }
            
            if args.iter().any(|a| a == "-z" || a == "--gzip") {
                plain.push_str(" (gzip compressed)");
            }
            if args.iter().any(|a| a == "-j" || a == "--bzip2") {
                plain.push_str(" (bzip2 compressed)");
            }
        }
        "ls" => {
            plain.push_str("List directory contents");
            if args.iter().any(|a| a == "-a" || a == "--all") {
                plain.push_str(" including hidden files");
            }
            if args.iter().any(|a| a == "-l") {
                plain.push_str(" with detailed information");
            }
            if args.iter().any(|a| a == "-R" || a == "--recursive") {
                plain.push_str(" recursively");
            }
        }
        "cat" => {
            plain.push_str("Display file contents");
        }
        "awk" => {
            // Try to detect what awk is doing
            let script_idx = args.iter().position(|a| a.starts_with("'{") || a.starts_with("{") || a.starts_with("'") || !a.starts_with('-'));
            if let Some(idx) = script_idx {
                let script = &args[idx];
                if script.contains("print") && script.contains("$1") {
                    plain.push_str("Print the first column");
                } else if script.contains("print") && script.contains("$2") {
                    if script.contains("sum") || script.contains("+=") {
                        plain.push_str("Sum up values in column 2");
                    } else {
                        plain.push_str("Print the second column");
                    }
                } else if script.contains("NF") {
                    plain.push_str("Process text fields");
                } else {
                    plain.push_str("Process text with a custom script");
                }
            } else {
                plain.push_str("Process text with awk");
            }
        }
        "sed" => {
            let script_idx = args.iter().position(|a| a.starts_with("'s/") || a.starts_with("s/"));
            if script_idx.is_some() {
                plain.push_str("Perform text substitution");
            } else {
                plain.push_str("Transform text with sed");
            }
        }
        "sort" => {
            plain.push_str("Sort lines of text");
            if args.iter().any(|a| a == "-n" || a == "--numeric-sort") {
                plain.push_str(" numerically");
            }
            if args.iter().any(|a| a == "-r" || a == "--reverse") {
                plain.push_str(" in reverse order");
            }
            if args.iter().any(|a| a == "-u" || a == "--unique") {
                plain.push_str(", removing duplicates");
            }
        }
        "uniq" => {
            plain.push_str("Filter duplicate lines");
            if args.iter().any(|a| a == "-c" || a == "--count") {
                plain.push_str(" and count occurrences");
            }
            if args.iter().any(|a| a == "-d" || a == "--repeated") {
                plain.push_str(", showing only duplicates");
            }
        }
        "wc" => {
            if args.iter().any(|a| a == "-l" || a == "--lines") {
                plain.push_str("Count lines");
            } else if args.iter().any(|a| a == "-w" || a == "--words") {
                plain.push_str("Count words");
            } else if args.iter().any(|a| a == "-c" || a == "--bytes") {
                plain.push_str("Count bytes");
            } else {
                plain.push_str("Count lines, words, and bytes");
            }
        }
        "head" => {
            plain.push_str("Show the beginning of a file");
        }
        "tail" => {
            plain.push_str("Show the end of a file");
            if args.iter().any(|a| a == "-f" || a == "--follow") {
                plain.push_str(" and follow new changes");
            }
        }
        "cut" => {
            plain.push_str("Extract specific columns/fields from text");
        }
        "tr" => {
            plain.push_str("Translate or delete characters");
        }
        "chmod" => {
            plain.push_str("Change file permissions");
        }
        "chown" => {
            plain.push_str("Change file ownership");
        }
        "curl" => {
            plain.push_str("Transfer data from/to a URL");
            if args.iter().any(|a| a == "-O" || a == "--remote-name") {
                plain.push_str(", saving to file");
            }
            if args.iter().any(|a| a == "-L" || a == "--location") {
                plain.push_str(", following redirects");
            }
        }
        "wget" => {
            plain.push_str("Download files from the web");
        }
        "ssh" => {
            plain.push_str("Connect to a remote server securely");
        }
        "scp" => {
            plain.push_str("Copy files securely between hosts");
        }
        "rsync" => {
            plain.push_str("Synchronize files between locations");
        }
        "xargs" => {
            plain.push_str("Build and execute commands from input");
        }
        "du" => {
            plain.push_str("Show disk usage");
        }
        "df" => {
            plain.push_str("Show free disk space");
        }
        "ps" => {
            plain.push_str("Show running processes");
        }
        "kill" => {
            plain.push_str("Send signal to a process");
        }
        "git" => {
            if let Some(subcmd) = args.first() {
                match subcmd.as_str() {
                    "add" => plain.push_str("Stage files for commit"),
                    "commit" => plain.push_str("Record changes to repository"),
                    "push" => plain.push_str("Upload commits to remote"),
                    "pull" => plain.push_str("Download from remote and merge"),
                    "clone" => plain.push_str("Copy a repository"),
                    "status" => plain.push_str("Show working tree status"),
                    "log" => plain.push_str("Show commit history"),
                    "branch" => plain.push_str("List or manage branches"),
                    "checkout" => plain.push_str("Switch branches"),
                    "merge" => plain.push_str("Join development histories"),
                    "diff" => plain.push_str("Show changes between commits"),
                    _ => plain.push_str("Execute git command"),
                }
            } else {
                plain.push_str("Work with git repository");
            }
        }
        "docker" => {
            if let Some(subcmd) = args.first() {
                match subcmd.as_str() {
                    "ps" => plain.push_str("List running containers"),
                    "run" => plain.push_str("Run a new container"),
                    "exec" => plain.push_str("Execute command in container"),
                    "build" => plain.push_str("Build an image"),
                    "images" => plain.push_str("List images"),
                    "pull" => plain.push_str("Download an image"),
                    "push" => plain.push_str("Upload an image"),
                    "rm" => plain.push_str("Remove containers"),
                    "rmi" => plain.push_str("Remove images"),
                    _ => plain.push_str("Execute docker command"),
                }
            } else {
                plain.push_str("Manage Docker containers");
            }
        }
        "rm" => {
            plain.push_str("Remove files or directories");
            if args.iter().any(|a| a == "-r" || a == "-R" || a == "--recursive") {
                plain.push_str(" recursively");
            }
            if args.iter().any(|a| a == "-f" || a == "--force") {
                plain.push_str(" without prompting");
            }
        }
        "cp" => {
            plain.push_str("Copy files or directories");
            if args.iter().any(|a| a == "-r" || a == "-R" || a == "--recursive") {
                plain.push_str(" recursively");
            }
        }
        "mv" => {
            plain.push_str("Move or rename files");
        }
        "mkdir" => {
            plain.push_str("Create directories");
            if args.iter().any(|a| a == "-p" || a == "--parents") {
                plain.push_str(" including parent directories");
            }
        }
        "diff" => {
            plain.push_str("Compare files and show differences");
        }
        "ln" => {
            if args.iter().any(|a| a == "-s" || a == "--symbolic") {
                plain.push_str("Create a symbolic link");
            } else {
                plain.push_str("Create a hard link");
            }
        }
        // "ps" handled earlier (docker context)
        "top" => {
            plain.push_str("Monitor system processes interactively");
        }
        "netstat" => {
            plain.push_str("Display network connections and statistics");
        }
        "iptables" => {
            plain.push_str("Configure firewall rules");
        }
        "crontab" => {
            plain.push_str("Manage scheduled tasks");
        }
        "ffmpeg" => {
            plain.push_str("Convert or process audio/video");
        }
        "make" => {
            plain.push_str("Build software from source");
        }
        "gcc" => {
            plain.push_str("Compile C code");
        }
        _ => {
            // Generic fallback
            if let Some(info) = build_knowledge_base().get(cmd) {
                plain.push_str(info.description);
            } else {
                plain.push_str(&format!("Execute '{}' command", cmd));
            }
        }
    }
    
    plain
}

pub fn execute(command_str: String, ctx: &CommandContext) -> Result<CommandOutput, EzError> {
    let kb = build_knowledge_base();
    let stages = parse_command(&command_str);

    if stages.is_empty() {
        return Err(EzError::InvalidArgs("Empty command".to_string()));
    }

    let is_pipeline = stages.len() > 1;

    if ctx.json {
        // JSON output
        let data = if is_pipeline {
            let mut stage_results = Vec::new();
            let mut full_command = String::new();

            for (idx, (cmd, args)) in stages.iter().enumerate() {
                let (breakdown, _) = explain_single_command(cmd, args, &kb);

                if idx > 0 {
                    full_command.push_str(" | ");
                }
                full_command.push_str(cmd);
                for arg in args {
                    full_command.push(' ');
                    full_command.push_str(arg);
                }

                stage_results.push(StageResult {
                    stage: idx + 1,
                    command: format!("{} {}", cmd, args.join(" ")).trim().to_string(),
                    breakdown,
                });
            }

            let result = ExplainResult {
                command: full_command,
                args: vec![],
                breakdown: vec![],
                plain_english: format!("Pipeline with {} stages", stages.len()),
                stages: Some(stage_results),
            };

            serde_json::to_value(&result).unwrap_or(serde_json::json!({}))
        } else {
            let (cmd, args) = &stages[0];
            let (breakdown, plain_english) = explain_single_command(cmd, args, &kb);

            let result = ExplainResult {
                command: cmd.clone(),
                args: args.clone(),
                breakdown,
                plain_english,
                stages: None,
            };

            serde_json::to_value(&result).unwrap_or(serde_json::json!({}))
        };

        return Ok(CommandOutput::new("explain", data));
    }

    // Pretty output
    if is_pipeline {
        println!("{}", "📖 Pipeline Breakdown:".bold().cyan());
        println!();

        for (idx, (cmd, args)) in stages.iter().enumerate() {
            let (breakdown, _) = explain_single_command(cmd, args, &kb);

            println!("  {} {}", "Stage".bold(), format!("{}", idx + 1).yellow().bold());

            for item in breakdown {
                let padded = format!("  {:<25}", item.part);
                println!("    {} {}", padded.yellow(), format!("→ {}", item.meaning).dimmed());
            }

            if idx < stages.len() - 1 {
                println!("    {}", "↓ (pipe to next stage)".dimmed());
            }
            println!();
        }

        println!("{} {}", "💡 In plain English:".bold().green(),
            format!("A pipeline of {} commands processing data through multiple stages", stages.len()));
    } else {
        let (cmd, args) = &stages[0];
        let (breakdown, plain_english) = explain_single_command(cmd, args, &kb);

        println!("{}", "📖 Command Breakdown:".bold().cyan());
        println!();

        for item in breakdown {
            let padded = format!("  {:<30}", item.part);
            println!("{} {}", padded.yellow(), format!("→ {}", item.meaning).dimmed());
        }

        println!();
        println!("{} {}", "💡 In plain English:".bold().green(), plain_english);
    }

    Ok(CommandOutput::new("explain", serde_json::json!({
        "command": command_str,
    })))
}
