use super::add::AddCommands;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Add new stuff
    Add {
        #[command(subcommand)]
        subcmd: AddCommands,
    },

    /// Update existing stuff
    Update {},
}
