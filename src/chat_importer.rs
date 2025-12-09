use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Copilot Chat Importer v{}", env!("CARGO_PKG_VERSION"));
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
        "--import" | "-i" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a file path to import");
                std::process::exit(1);
            }
            import_chat(&args[2]);
        }
        "--export" | "-e" => {
            if args.len() < 4 {
                eprintln!("Error: Please provide input and output file paths");
                std::process::exit(1);
            }
            export_chat(&args[2], &args[3]);
        }
        "--analyze" | "-a" => {
            if args.len() < 3 {
                eprintln!("Error: Please provide a file path to analyze");
                std::process::exit(1);
            }
            analyze_chat(&args[2]);
        }
        _ => {
            eprintln!("Unknown option: {}", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("A tool for importing and managing Copilot chat conversations.");
    println!();
    println!("Usage:");
    println!("  copilot-chat-importer [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -h, --help                    Display this help message");
    println!("  -v, --version                 Display version information");
    println!("  -i, --import <file>           Import a chat conversation file");
    println!("  -e, --export <in> <out>       Export chat to a different format");
    println!("  -a, --analyze <file>          Analyze chat conversation statistics");
    println!();
    println!("Examples:");
    println!("  copilot-chat-importer --import chat.txt");
    println!("  copilot-chat-importer --export chat.txt chat.md");
    println!("  copilot-chat-importer --analyze chat.txt");
}

fn import_chat(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    }
    
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("Importing chat from: {}", file_path);
            println!("{}", "=".repeat(60));
            
            let lines: Vec<&str> = content.lines().collect();
            let mut message_count = 0;
            let mut current_speaker = "";
            
            for line in &lines {
                if line.starts_with("User:") || line.starts_with("user:") {
                    message_count += 1;
                    current_speaker = "User";
                    println!("\n[{}] {}", current_speaker, &line[5..].trim());
                } else if line.starts_with("Copilot:") || line.starts_with("copilot:") {
                    message_count += 1;
                    current_speaker = "Copilot";
                    println!("\n[{}] {}", current_speaker, &line[8..].trim());
                } else if line.starts_with("Assistant:") || line.starts_with("assistant:") {
                    message_count += 1;
                    current_speaker = "Assistant";
                    println!("\n[{}] {}", current_speaker, &line[10..].trim());
                } else if !line.trim().is_empty() {
                    println!("  {}", line);
                }
            }
            
            println!("\n{}", "=".repeat(60));
            println!("\nChat imported successfully!");
            println!("Total messages: {}", message_count);
            println!("Total lines: {}", lines.len());
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn export_chat(input_path: &str, output_path: &str) {
    let in_path = Path::new(input_path);
    
    if !in_path.exists() {
        eprintln!("Error: Input file not found: {}", input_path);
        std::process::exit(1);
    }
    
    match fs::read_to_string(in_path) {
        Ok(content) => {
            println!("Exporting chat from: {} to: {}", input_path, output_path);
            
            let lines: Vec<&str> = content.lines().collect();
            let mut markdown = String::new();
            markdown.push_str("# Copilot Chat Export\n\n");
            
            for line in lines {
                if line.starts_with("User:") || line.starts_with("user:") {
                    markdown.push_str(&format!("\n## 👤 User\n\n{}\n", &line[5..].trim()));
                } else if line.starts_with("Copilot:") || line.starts_with("copilot:") {
                    markdown.push_str(&format!("\n## 🤖 Copilot\n\n{}\n", &line[8..].trim()));
                } else if line.starts_with("Assistant:") || line.starts_with("assistant:") {
                    markdown.push_str(&format!("\n## 🤖 Assistant\n\n{}\n", &line[10..].trim()));
                } else if !line.trim().is_empty() {
                    markdown.push_str(&format!("{}\n", line));
                }
            }
            
            match fs::write(output_path, markdown) {
                Ok(_) => {
                    println!("Chat exported successfully to: {}", output_path);
                }
                Err(e) => {
                    eprintln!("Error writing file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}

fn analyze_chat(file_path: &str) {
    let path = Path::new(file_path);
    
    if !path.exists() {
        eprintln!("Error: File not found: {}", file_path);
        std::process::exit(1);
    }
    
    match fs::read_to_string(path) {
        Ok(content) => {
            println!("Analyzing chat from: {}", file_path);
            println!("{}", "=".repeat(60));
            
            let lines: Vec<&str> = content.lines().collect();
            let mut user_messages = 0;
            let mut copilot_messages = 0;
            let mut total_words = 0;
            let mut code_blocks = 0;
            
            for line in &lines {
                if line.starts_with("User:") || line.starts_with("user:") {
                    user_messages += 1;
                } else if line.starts_with("Copilot:") || line.starts_with("copilot:") || 
                          line.starts_with("Assistant:") || line.starts_with("assistant:") {
                    copilot_messages += 1;
                }
                
                if line.contains("```") {
                    code_blocks += 1;
                }
                
                total_words += line.split_whitespace().count();
            }
            
            println!("\nChat Statistics:");
            println!("{}", "-".repeat(60));
            println!("  Total lines:         {}", lines.len());
            println!("  User messages:       {}", user_messages);
            println!("  Copilot messages:    {}", copilot_messages);
            println!("  Total messages:      {}", user_messages + copilot_messages);
            println!("  Total words:         {}", total_words);
            println!("  Code blocks:         {}", code_blocks / 2); // Divide by 2 for open/close pairs
            println!("  Characters:          {}", content.len());
            println!("{}", "=".repeat(60));
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }
}
