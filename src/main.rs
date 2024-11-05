use clap::Parser;
use std::process;

mod cli;
mod db;
mod service;

use cli::handler::CommandHandler;
use cli::types::Cli;

fn main() {
    let cli = Cli::parse();

    // Make it configurable later on
    let database_url = "doit.db";

    let mut handler = match CommandHandler::new(database_url) {
        Ok(handler) => handler,
        Err(e) => {
            eprintln!("Failed to initialize command handler: {}", e);
            process::exit(1);
        }
    };

    match handler.handle_command(cli) {
        Ok(message) => println!("{}", message),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}
