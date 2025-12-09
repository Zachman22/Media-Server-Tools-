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

### 4. Auto Programmer ⭐ NEW
AI-powered code generation tool for multiple programming languages. **Trusted & Signed** code templates.

**Usage:**
```bash
auto-programmer [OPTIONS]
```

**Options:**
- `-h, --help` - Display help message
- `-v, --version` - Display version information
- `-g, --generate <lang> <file>` - Generate code template for language
- `-p, --prompt <lang>` - Generate AI prompt for language
- `-l, --languages` - List supported languages
- `-s, --scaffold <type> <name>` - Scaffold a new project

**Supported Languages:**
Python, JavaScript, TypeScript, Rust, Go, Bash, PowerShell, C, C++, Java, C#, Ruby, PHP

**Examples:**
```bash
# Generate a Python script template
auto-programmer --generate python script.py

# Generate AI prompt for Rust development
auto-programmer --prompt rust

# Scaffold a new web project
auto-programmer --scaffold web my-app

# List all supported languages
auto-programmer --languages
```

**Features:**
- ✓ Trusted and signed code templates
- ✓ Best practices and proper error handling
- ✓ AI prompt generation for any language
- ✓ Project scaffolding for web, CLI, API, and library projects
- ✓ Support for 13+ programming languages

### 5. Media Dashboard (C++) ⭐ NEW - Beautiful EXE
A stunning C++ application with beautiful terminal UI for media server management.

**Usage:**
```bash
media-dashboard [OPTIONS]
```

**Options:**
- `-h, --help` - Display help message
- `-v, --version` - Display version information

**Features:**
- 🎨 Beautiful terminal UI with colors and box-drawing characters
- 📊 Interactive dashboard with statistics
- 🎬 Media library browser
- 🌐 Active connections monitor
- 💻 System information display
- ⚡ High-performance C++17 code
- 🎯 Native x64 executable

**Interactive Menu:**
1. View Dashboard Statistics
2. Browse Media Library
3. Active Connections
4. System Information
5. Server Status
0. Exit

Run `media-dashboard` to launch the beautiful interactive interface!

## Building

All tools are built as x64 executables for Windows and Linux.

**Build Rust tools:**
```bash
cargo build --release
```

**Build C++ Dashboard:**
```bash
cd cpp
./build.sh
```

**Executables will be located in:**

**Rust Tools:**
- `target/release/media-server-tools` (or `.exe` on Windows)
- `target/release/copilot-code-sharer` (or `.exe` on Windows)
- `target/release/copilot-chat-importer` (or `.exe` on Windows)
- `target/release/auto-programmer` (or `.exe` on Windows)

**C++ Dashboard:**
- `cpp/build/bin/media-dashboard` (or `.exe` on Windows)

## CI/CD

GitHub Actions automatically builds x64 executables for:
- Windows (x86_64-pc-windows-msvc)
- Linux (x86_64-unknown-linux-gnu)

Artifacts are available for download after each successful build.

