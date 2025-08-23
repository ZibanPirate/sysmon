use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lib-swift")]
#[command(about = "A CLI tool to test native macOS code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get screen information
    ScreenInfo,
    /// Get network information
    NetworkInfo,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::ScreenInfo => {
            println!("=== Screen Info ===");
            let screen_info = lib_swift::get_screen_info();
            println!("Screen Info: {:#?}", screen_info);
        }
        Commands::NetworkInfo => {
            println!("=== Network Info ===");
            let network_info = lib_swift::get_network_info();
            println!("Network Info: {:#?}", network_info);
        }
    }
}
