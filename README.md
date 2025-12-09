# Media-Server-Tools

A collection of command-line tools for media server management and Copilot development workflows.

## Tools Included

### 1. Media Server Tools
A simple media server management tool.

**Usage:**
```bash
media-server-tools [OPTIONS]
```

**Options:**
- `--version` - Display version information
- `--help` - Display help message

### 2. Copilot Code Sharer
A tool for sharing code snippets and files.

**Usage:**
```bash
copilot-code-sharer [OPTIONS]
```

**Options:**
- `-h, --help` - Display help message
- `-v, --version` - Display version information
- `-s, --share <file>` - Share a code file
- `-l, --list <dir>` - List all code files in a directory

**Examples:**
```bash
copilot-code-sharer --share main.rs
copilot-code-sharer --list ./src
```

### 3. Copilot Chat Importer
A tool for importing and managing Copilot chat conversations.

**Usage:**
```bash
copilot-chat-importer [OPTIONS]
```

**Options:**
- `-h, --help` - Display help message
- `-v, --version` - Display version information
- `-i, --import <file>` - Import a chat conversation file
- `-e, --export <in> <out>` - Export chat to a different format
- `-a, --analyze <file>` - Analyze chat conversation statistics

**Examples:**
```bash
copilot-chat-importer --import chat.txt
copilot-chat-importer --export chat.txt chat.md
copilot-chat-importer --analyze chat.txt
```

## Building

All tools are built as x64 executables for Windows and Linux.

**Build locally:**
```bash
cargo build --release
```

**Executables will be located in:**
- `target/release/media-server-tools` (or `.exe` on Windows)
- `target/release/copilot-code-sharer` (or `.exe` on Windows)
- `target/release/copilot-chat-importer` (or `.exe` on Windows)

## CI/CD

GitHub Actions automatically builds x64 executables for:
- Windows (x86_64-pc-windows-msvc)
- Linux (x86_64-unknown-linux-gnu)

Artifacts are available for download after each successful build.

