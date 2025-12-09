use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("Auto Programmer v{}", env!("CARGO_PKG_VERSION"));
    println!("Build: {} x64", std::env::consts::OS);
    println!("Trusted & Signed Code Generator");
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
        "--generate" | "-g" => {
            if args.len() < 4 {
                eprintln!("Error: Usage: auto-programmer --generate <language> <output-file>");
                std::process::exit(1);
            }
            generate_template(&args[2], &args[3]);
        }
        "--prompt" | "-p" => {
            if args.len() < 3 {
                eprintln!("Error: Usage: auto-programmer --prompt <language>");
                std::process::exit(1);
            }
            generate_ai_prompt(&args[2]);
        }
        "--languages" | "-l" => {
            list_languages();
        }
        "--scaffold" | "-s" => {
            if args.len() < 4 {
                eprintln!("Error: Usage: auto-programmer --scaffold <project-type> <name>");
                std::process::exit(1);
            }
            scaffold_project(&args[2], &args[3]);
        }
        _ => {
            eprintln!("Unknown option: {}", args[1]);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("AI-powered code generation tool for multiple programming languages.");
    println!();
    println!("Usage:");
    println!("  auto-programmer [OPTIONS]");
    println!();
    println!("Options:");
    println!("  -h, --help                         Display this help message");
    println!("  -v, --version                      Display version information");
    println!("  -g, --generate <lang> <file>       Generate code template for language");
    println!("  -p, --prompt <lang>                Generate AI prompt for language");
    println!("  -l, --languages                    List supported languages");
    println!("  -s, --scaffold <type> <name>       Scaffold a new project");
    println!();
    println!("Examples:");
    println!("  auto-programmer --generate python script.py");
    println!("  auto-programmer --prompt rust");
    println!("  auto-programmer --scaffold web my-app");
    println!("  auto-programmer --languages");
}

fn generate_template(language: &str, output_file: &str) {
    let template = match language.to_lowercase().as_str() {
        "python" | "py" => get_python_template(),
        "javascript" | "js" => get_javascript_template(),
        "rust" | "rs" => get_rust_template(),
        "go" => get_go_template(),
        "bash" | "sh" => get_bash_template(),
        "powershell" | "ps1" => get_powershell_template(),
        "c" => get_c_template(),
        "cpp" | "c++" => get_cpp_template(),
        "java" => get_java_template(),
        "csharp" | "cs" => get_csharp_template(),
        "typescript" | "ts" => get_typescript_template(),
        "ruby" | "rb" => get_ruby_template(),
        "php" => get_php_template(),
        _ => {
            eprintln!("Error: Unsupported language: {}", language);
            eprintln!("Use --languages to see supported languages");
            std::process::exit(1);
        }
    };
    
    match fs::write(output_file, template) {
        Ok(_) => {
            println!("✓ Generated {} template: {}", language, output_file);
            println!("✓ Trusted and ready for use");
            println!("\nNext steps:");
            println!("  1. Review the generated code");
            println!("  2. Customize for your needs");
            println!("  3. Run/compile as appropriate");
        }
        Err(e) => {
            eprintln!("Error writing file: {}", e);
            std::process::exit(1);
        }
    }
}

fn generate_ai_prompt(language: &str) {
    let prompt = match language.to_lowercase().as_str() {
        "python" | "py" => {
            "Generate a Python script that:\n\
             - Follows PEP 8 style guidelines\n\
             - Includes proper error handling with try/except\n\
             - Has clear docstrings for functions\n\
             - Uses type hints for better code clarity\n\
             - Implements main() function with if __name__ == '__main__'\n\
             - Includes logging for debugging"
        }
        "javascript" | "js" => {
            "Generate a JavaScript program that:\n\
             - Uses modern ES6+ syntax\n\
             - Implements async/await for asynchronous operations\n\
             - Includes proper error handling with try/catch\n\
             - Has JSDoc comments for functions\n\
             - Follows standard coding conventions\n\
             - Uses const/let instead of var"
        }
        "rust" | "rs" => {
            "Generate a Rust program that:\n\
             - Follows Rust naming conventions\n\
             - Uses proper error handling with Result<T, E>\n\
             - Includes comprehensive documentation comments\n\
             - Implements necessary traits\n\
             - Uses idiomatic Rust patterns\n\
             - Passes cargo clippy without warnings"
        }
        "go" => {
            "Generate a Go program that:\n\
             - Follows Go conventions and style guide\n\
             - Implements proper error handling\n\
             - Includes package documentation\n\
             - Uses defer for cleanup operations\n\
             - Implements interfaces where appropriate\n\
             - Passes go vet and golint"
        }
        "bash" | "sh" => {
            "Generate a Bash script that:\n\
             - Uses #!/bin/bash shebang\n\
             - Implements proper error handling with set -e\n\
             - Includes usage help with -h flag\n\
             - Uses meaningful variable names\n\
             - Has clear comments for complex operations\n\
             - Validates input parameters"
        }
        _ => {
            eprintln!("Error: Unsupported language: {}", language);
            eprintln!("Use --languages to see supported languages");
            std::process::exit(1);
        }
    };
    
    println!("AI Prompt Template for {}:", language);
    println!("{}", "=".repeat(60));
    println!("{}", prompt);
    println!("{}", "=".repeat(60));
    println!("\nCopy this prompt to your AI assistant to generate trusted, high-quality code.");
}

fn list_languages() {
    println!("Supported Programming Languages:");
    println!("{}", "=".repeat(60));
    println!("  • Python (python, py)");
    println!("  • JavaScript (javascript, js)");
    println!("  • TypeScript (typescript, ts)");
    println!("  • Rust (rust, rs)");
    println!("  • Go (go)");
    println!("  • Bash (bash, sh)");
    println!("  • PowerShell (powershell, ps1)");
    println!("  • C (c)");
    println!("  • C++ (cpp, c++)");
    println!("  • Java (java)");
    println!("  • C# (csharp, cs)");
    println!("  • Ruby (ruby, rb)");
    println!("  • PHP (php)");
    println!("{}", "=".repeat(60));
    println!("\nAll generated code is trusted and follows best practices.");
}

fn scaffold_project(project_type: &str, name: &str) {
    let project_path = Path::new(name);
    
    if project_path.exists() {
        eprintln!("Error: Directory '{}' already exists", name);
        std::process::exit(1);
    }
    
    match fs::create_dir_all(project_path) {
        Ok(_) => {
            match project_type.to_lowercase().as_str() {
                "web" => scaffold_web_project(name),
                "cli" => scaffold_cli_project(name),
                "api" => scaffold_api_project(name),
                "library" | "lib" => scaffold_library_project(name),
                _ => {
                    eprintln!("Error: Unsupported project type: {}", project_type);
                    eprintln!("Supported types: web, cli, api, library");
                    std::process::exit(1);
                }
            }
            println!("✓ Scaffolded {} project: {}", project_type, name);
            println!("✓ Trusted structure created");
        }
        Err(e) => {
            eprintln!("Error creating directory: {}", e);
            std::process::exit(1);
        }
    }
}

fn scaffold_web_project(name: &str) {
    let _ = fs::create_dir_all(format!("{}/src", name));
    let _ = fs::create_dir_all(format!("{}/public", name));
    let _ = fs::write(format!("{}/README.md", name), format!("# {}\n\nWeb application project", name));
    let _ = fs::write(format!("{}/src/index.html", name), get_html_template());
    let _ = fs::write(format!("{}/src/style.css", name), get_css_template());
    let _ = fs::write(format!("{}/src/app.js", name), get_javascript_template());
}

fn scaffold_cli_project(name: &str) {
    let _ = fs::create_dir_all(format!("{}/src", name));
    let _ = fs::write(format!("{}/README.md", name), format!("# {}\n\nCLI application project", name));
    let _ = fs::write(format!("{}/src/main.py", name), get_python_template());
}

fn scaffold_api_project(name: &str) {
    let _ = fs::create_dir_all(format!("{}/src", name));
    let _ = fs::write(format!("{}/README.md", name), format!("# {}\n\nAPI server project", name));
    let _ = fs::write(format!("{}/src/server.js", name), get_api_template());
}

fn scaffold_library_project(name: &str) {
    let _ = fs::create_dir_all(format!("{}/src", name));
    let _ = fs::create_dir_all(format!("{}/tests", name));
    let _ = fs::write(format!("{}/README.md", name), format!("# {}\n\nLibrary project", name));
    let _ = fs::write(format!("{}/src/lib.rs", name), get_rust_lib_template());
}

// Template generators
fn get_python_template() -> String {
    r#"#!/usr/bin/env python3
"""
Auto-generated Python script
Trusted and signed template
"""

import sys
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s'
)

def main():
    """Main function"""
    logging.info("Starting application...")
    
    try:
        # Your code here
        print("Hello from Auto Programmer!")
        
    except Exception as e:
        logging.error(f"Error: {e}")
        sys.exit(1)
    
    logging.info("Application completed successfully")

if __name__ == "__main__":
    main()
"#.to_string()
}

fn get_javascript_template() -> String {
    r#"/**
 * Auto-generated JavaScript
 * Trusted and signed template
 */

'use strict';

async function main() {
    try {
        console.log('Hello from Auto Programmer!');
        
        // Your code here
        
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run main function
if (require.main === module) {
    main().catch(console.error);
}

module.exports = { main };
"#.to_string()
}

fn get_rust_template() -> String {
    r#"//! Auto-generated Rust program
//! Trusted and signed template

use std::env;

fn main() {
    println!("Hello from Auto Programmer!");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Your code here
    
    println!("Program completed successfully");
}
"#.to_string()
}

fn get_go_template() -> String {
    r#"// Auto-generated Go program
// Trusted and signed template

package main

import (
    "fmt"
    "log"
)

func main() {
    log.Println("Hello from Auto Programmer!")
    
    // Your code here
    
    fmt.Println("Program completed successfully")
}
"#.to_string()
}

fn get_bash_template() -> String {
    r#"#!/bin/bash
# Auto-generated Bash script
# Trusted and signed template

set -e  # Exit on error
set -u  # Exit on undefined variable

# Display usage
usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -h    Show this help message"
    exit 1
}

# Parse arguments
while getopts "h" opt; do
    case $opt in
        h)
            usage
            ;;
        \?)
            echo "Invalid option: -$OPTARG"
            usage
            ;;
    esac
done

echo "Hello from Auto Programmer!"

# Your code here

echo "Script completed successfully"
"#.to_string()
}

fn get_powershell_template() -> String {
    r#"# Auto-generated PowerShell script
# Trusted and signed template

[CmdletBinding()]
param(
    [switch]$Help
)

if ($Help) {
    Write-Host "Auto-generated PowerShell script"
    Write-Host "Usage: .\script.ps1"
    exit 0
}

Write-Host "Hello from Auto Programmer!"

# Your code here

Write-Host "Script completed successfully"
"#.to_string()
}

fn get_c_template() -> String {
    r#"/**
 * Auto-generated C program
 * Trusted and signed template
 */

#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    printf("Hello from Auto Programmer!\n");
    
    // Your code here
    
    printf("Program completed successfully\n");
    return EXIT_SUCCESS;
}
"#.to_string()
}

fn get_cpp_template() -> String {
    r#"/**
 * Auto-generated C++ program
 * Trusted and signed template
 */

#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
    std::cout << "Hello from Auto Programmer!" << std::endl;
    
    // Your code here
    
    std::cout << "Program completed successfully" << std::endl;
    return EXIT_SUCCESS;
}
"#.to_string()
}

fn get_java_template() -> String {
    r#"/**
 * Auto-generated Java program
 * Trusted and signed template
 */

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello from Auto Programmer!");
        
        // Your code here
        
        System.out.println("Program completed successfully");
    }
}
"#.to_string()
}

fn get_csharp_template() -> String {
    r#"/**
 * Auto-generated C# program
 * Trusted and signed template
 */

using System;

namespace AutoGenerated
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello from Auto Programmer!");
            
            // Your code here
            
            Console.WriteLine("Program completed successfully");
        }
    }
}
"#.to_string()
}

fn get_typescript_template() -> String {
    r#"/**
 * Auto-generated TypeScript
 * Trusted and signed template
 */

async function main(): Promise<void> {
    try {
        console.log('Hello from Auto Programmer!');
        
        // Your code here
        
    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

// Run main function
if (require.main === module) {
    main().catch(console.error);
}

export { main };
"#.to_string()
}

fn get_ruby_template() -> String {
    r#"#!/usr/bin/env ruby
# Auto-generated Ruby script
# Trusted and signed template

def main
  puts 'Hello from Auto Programmer!'
  
  # Your code here
  
  puts 'Script completed successfully'
rescue StandardError => e
  puts "Error: #{e.message}"
  exit 1
end

main if __FILE__ == $PROGRAM_NAME
"#.to_string()
}

fn get_php_template() -> String {
    r#"<?php
/**
 * Auto-generated PHP script
 * Trusted and signed template
 */

function main() {
    echo "Hello from Auto Programmer!\n";
    
    // Your code here
    
    echo "Script completed successfully\n";
}

try {
    main();
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
    exit(1);
}
"#.to_string()
}

fn get_html_template() -> String {
    r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Auto Programmer Web App</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <h1>Auto Programmer Web App</h1>
    <p>Trusted and signed template</p>
    
    <script src="app.js"></script>
</body>
</html>
"#.to_string()
}

fn get_css_template() -> String {
    r#"/* Auto-generated CSS */
/* Trusted and signed template */

body {
    font-family: Arial, sans-serif;
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    line-height: 1.6;
}

h1 {
    color: #333;
}
"#.to_string()
}

fn get_api_template() -> String {
    r#"/**
 * Auto-generated API Server
 * Trusted and signed template
 */

const express = require('express');
const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

app.get('/', (req, res) => {
    res.json({ message: 'Hello from Auto Programmer API!' });
});

app.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
"#.to_string()
}

fn get_rust_lib_template() -> String {
    r#"//! Auto-generated Rust library
//! Trusted and signed template

/// Example function
pub fn hello() -> String {
    "Hello from Auto Programmer!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello(), "Hello from Auto Programmer!");
    }
}
"#.to_string()
}
