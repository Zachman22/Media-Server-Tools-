use std::env;

fn main() {
    println!("Media Server Tools v{}", env!("CARGO_PKG_VERSION"));
    println!("Build: {} x64", std::env::consts::OS);
    println!();
    println!("A simple media server management tool.");
    println!();
    println!("Usage:");
    println!("  media-server-tools [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --version    Display version information");
    println!("  --help       Display this help message");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("\nVersion: {}", env!("CARGO_PKG_VERSION"));
            }
            "--help" | "-h" => {
                // Already printed help above
            }
            _ => {
                println!("\nUnknown option: {}", args[1]);
                println!("Use --help for usage information");
            }
        }
    }
}
