mod api;
mod cli;
mod client;
mod core;
mod llm;

use api::server::start_server;
use clap::Parser;
use cli::commands::{Cli, Commands};
use core::engine::process_natural_language_query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { prompt } => {
            println!("Processing query...");
            match process_natural_language_query(&prompt).await {
                Ok(summary) => println!("\nResult\n{}", summary),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Serve { port } => {
            start_server(port).await?;
        }
    }
    Ok(())
}
