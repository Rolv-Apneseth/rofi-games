use std::{env, io, path::Path};

mod desktop_entries;
mod helpers;
mod steam;

use desktop_entries::DesktopEntriesControl;

fn main() -> Result<(), io::Error> {
    let dir_steam = Path::new(env!("HOME")).join(".local/share/Steam");
    let dir_desktop_files = Path::new(env!("XDG_DATA_HOME")).join("applications/rofi_games");

    let control = DesktopEntriesControl::new(dir_steam, dir_desktop_files)?;

    // control.delete_all_entries()?;
    // control.reset_entries()?;
    control.sync_entries()?;

    Ok(())
}
