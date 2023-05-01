use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Delete all desktop entries created by this program
    Delete,
    /// Hard reset all desktop entries (delete + regenerate)
    Reset,
    /// Sync desktop entries (remove entries without a match + generate new entries for games which
    /// don't have a match)
    Sync,
}
