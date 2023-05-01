use clap::Parser;
use std::{env, io, path::Path};

mod cli;
mod desktop_entries;
mod helpers;
mod steam;

use cli::{Cli, Command};
use desktop_entries::DesktopEntriesControl;

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    let dir_steam = Path::new(env!("HOME")).join(".local/share/Steam");
    let dir_desktop_files = Path::new(env!("XDG_DATA_HOME")).join("applications/rofi_games");

    let control = DesktopEntriesControl::new(dir_steam, dir_desktop_files)?;

    // Execute command
    match &cli.command {
        Command::Delete => {
            control.delete_all_entries()?;
            println!("\nDesktop entries deleted successfully")
        }
        Command::Reset => {
            control.reset_entries()?;
            println!("\nDesktop entries reset successfully")
        }
        Command::Sync => {
            control.sync_entries()?;
            println!("\nDesktop entries synced successfully")
        }
    };

    Ok(())
}
