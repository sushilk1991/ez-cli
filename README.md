# ez - Easy Unix

> User-friendly replacements for common Unix commands

`ez` is a Rust CLI that makes Unix commands more intuitive and user-friendly. Instead of cryptic flags and terse output, `ez` uses natural language commands with helpful, colorized output.

## 🚀 Installation

### From Source
```bash
git clone https://github.com/sushilk1991/ez-cli
cd ez-cli
cargo build --release

# Install to ~/.cargo/bin
cargo install --path .
```

### Prerequisites
- Rust 1.70 or later
- Linux/macOS/Unix system

## 📖 Commands

| ez Command | Unix Equivalent | Description |
|------------|-----------------|-------------|
| `ez list` | `ls` | List files with icons and colors |
| `ez show` | `cat` | Display file contents |
| `ez find` | `find`/`grep` | Find files or search in files |
| `ez copy` | `cp` | Copy files/folders with progress |
| `ez move` | `mv` | Move/rename files |
| `ez remove` | `rm` | Remove files/folders safely |
| `ez create-folder` | `mkdir` | Create directories |
| `ez create-file` | `touch` | Create empty files |
| `ez where` | `pwd` | Show current directory |
| `ez size` | `du` | Show folder sizes |
| `ez running` | `ps` | Show running processes |
| `ez stop` | `kill` | Stop processes |
| `ez download` | `curl`/`wget` | Download files |
| `ez pack` | `tar`/`zip` | Create archives |
| `ez unpack` | `tar`/`unzip` | Extract archives |
| `ez space` | `df` | Show disk space |
| `ez count` | `wc` | Count lines/words/bytes |
| `ez sort` | `sort` | Sort file contents |
| `ez compare` | `diff` | Compare files |
| `ez make-runnable` | `chmod +x` | Make files executable |
| `ez help-me` | `man` | Show help and examples |

## 💡 Examples

### Listing Files
```bash
# Simple listing with icons and colors
ez list

# Detailed view with sizes and dates
ez list --details

# Show hidden files
ez list --all

# Sort by modification time
ez list --time
```

### Finding Files
```bash
# Find files by name
ez find "*.rs"

# Search inside files (like grep)
ez find "TODO" --inside

# Case-insensitive search with line numbers
ez find "error" --inside --ignore-case --line-numbers
```

### Working with Files
```bash
# Copy with progress bar
ez copy large_file.zip backup/ --progress

# Copy directories recursively
ez copy folder/ backup/ --recursive

# Remove with confirmation (safer than rm!)
ez remove folder/ --recursive

# Force remove without asking
ez remove temp/ --recursive --force
```

### Viewing Files
```bash
# Show file with line numbers
ez show code.py --numbers

# Show first 20 lines
ez show log.txt --first 20

# Show last 50 lines
ez show log.txt --last 50
```

### Archives
```bash
# Create zip archive
ez pack backup.zip file1.txt file2.txt folder/

# Create tar.gz archive
ez pack backup.tar.gz myfolder/

# Extract archive
ez unpack backup.tar.gz --to ./extracted/
```

### System Information
```bash
# Show folder size
ez size /var/log --detailed

# Show disk space
ez space

# Show running processes
ez running --all

# Filter processes
ez running --filter chrome

# Stop a process
ez stop 1234
# or by name
ez stop firefox
```

### Downloading Files
```bash
# Download with progress bar
ez download https://example.com/file.zip --progress

# Download and save with specific name
ez download https://example.com/data.json --save mydata.json
```

### Comparing Files
```bash
# Unified diff view
ez compare file1.txt file2.txt

# Side-by-side comparison
ez compare file1.txt file2.txt --side-by-side
```

## 🎨 Features

- **🎯 Natural Language Commands**: Use `copy` instead of `cp`, `remove` instead of `rm`
- **🌈 Colorized Output**: Beautiful, easy-to-read output with icons and colors
- **🛡️ Safe by Default**: Confirmations before destructive actions
- **📊 Progress Bars**: Visual feedback for long operations
- **🔍 Helpful Errors**: Clear error messages instead of cryptic Unix errors
- **📦 Archive Support**: Zip, Tar, Tar.gz, Tar.bz2 support
- **🔎 Smart Search**: Find files or search inside them with one command

## 🆚 Comparison with Unix Commands

### Before (Unix)
```bash
$ ls -lah --sort=time
-rw-r--r--  1 user group  1.2K Jan 15 10:30 README.md
-rw-r--r--  1 user group  4.5K Jan 14 09:15 main.rs
```

### After (ez)
```bash
$ ez list --details --time
📄    1.2 KB   Jan 15 10:30 README.md
📄    4.5 KB   Jan 14 09:15 main.rs
```

### Before (Unix)
```bash
$ ps aux | grep chrome
user    1234  5.2  2.1  45234  8923 ?   Sl   Jan14   2:34 chrome
```

### After (ez)
```bash
$ ez running --filter chrome
  PID   %CPU  %MEM  COMMAND
 1234    5.2   2.1  chrome
```

## 🔧 Configuration

No configuration needed! `ez` works out of the box with sensible defaults.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Inspired by the need for more user-friendly Unix tools
- Built with Rust and love ❤️
- Thanks to all contributors!

---

**Made with 🦉 by Vic**
