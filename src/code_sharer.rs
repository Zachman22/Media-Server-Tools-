use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Copilot Code Sharer v{}", env!("CARGO_PKG_VERSION"));
    println!("Build: {} x64", std::env::consts::OS);
    println!();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }
        "--version" | "-v" => {
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }
        "--share" | "-s" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a file path to share");
                std::process::exit(1);
            }
            share_code(&args[2]);
        }
        "--list" | "-l" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a directory path to list");
                std::process::exit(1);
            }
            list_files(&args[2]);
        }
        _ => {
            eprintln!("Unknown option: {}", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("A tool for sharing code snippets and files.");
    println!();
    println!("Usage:");
    println!("  copilot-code-sharer [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -h, --help           Display this help message");
    println!("  -v, --version        Display version information");
    println!("  -s, --share <file>   Share a code file");
    println!("  -l, --list <dir>     List all code files in a directory");
    println!();
    println!("Examples:");
    println!("  copilot-code-sharer --share main.rs");
    println!("  copilot-code-sharer --list ./src");
}

fn share_code(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    }
    
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("Sharing code from: {}", file_path);
            println!("{}", "=".repeat(60));
            println!("{}", content);
            println!("{}", "=".repeat(60));
            println!("\nFile shared successfully!");
            println!("Lines: {}", content.lines().count());
            println!("Characters: {}", content.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn list_files(dir_path: &str) {
    let path = Path::new(dir_path);
    
    if !path.exists() {
        eprintln!("Error: Directory not found: {}", dir_path);
        std::process::exit(1);
    }
    
    if !path.is_dir() {
        eprintln!("Error: Not a directory: {}", dir_path);
        std::process::exit(1);
    }
    
    println!("Code files in: {}", dir_path);
    println!("{}", "-".repeat(60));
    
    match fs::read_dir(path) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(extension) = path.extension() {
                            let ext = extension.to_string_lossy();
                            if is_code_file(&ext) {
                                count += 1;
                                println!("  {} ({})", 
                                    path.file_name().unwrap().to_string_lossy(),
                                    ext
                                );
                            }
                        }
                    }
                }
            }
            println!("{}", "-".repeat(60));
            println!("Total code files found: {}", count);
        }
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
            std::process::exit(1);
        }
    }
}

fn is_code_file(extension: &str) -> bool {
    matches!(
        extension,
        "rs" | "c" | "cpp" | "h" | "hpp" | "cs" | "java" | "py" | "js" | "ts" | "go" | "rb" | 
        "php" | "swift" | "kt" | "scala" | "r" | "m" | "mm" | "sh" | "bash" | "ps1" | "toml" |
        "yaml" | "yml" | "json" | "xml" | "html" | "css" | "sql" | "md"
    )
}
