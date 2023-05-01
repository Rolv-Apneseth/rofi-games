use std::{
    collections::HashSet,
    fs::{create_dir, read_dir, remove_dir_all, remove_file, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::{
    helpers::get_str_from_path_buf,
    steam::{get_steam_libraries, SteamGame},
};

pub struct DesktopEntriesControl {
    dir_steam: PathBuf,
    dir_desktop_files: PathBuf,
    games: Vec<SteamGame>,
}
impl DesktopEntriesControl {
    /// Returns a new DesktopEntriesControl which will act on the given dir_steam and
    /// dir_desktop_files paths
    pub fn new(
        dir_steam: PathBuf,
        dir_desktop_files: PathBuf,
    ) -> Result<DesktopEntriesControl, io::Error> {
        // Get all steam libraries, and games in those libraries
        let libraries = get_steam_libraries(&dir_steam)?;
        let mut games: Vec<SteamGame> = Vec::new();

        for library in libraries {
            games.append(&mut library.get_games()?);
        }

        Ok(DesktopEntriesControl {
            dir_steam,
            dir_desktop_files,
            games,
        })
    }

    /// Syncs desktop entries by removing old entries which no longer have associated games and
    /// creating new entries for games which don't have one
    pub fn sync_entries(&self) -> Result<(), io::Error> {
        self.ensure_dir_desktop_files()?;

        self.delete_dead_entries()?;
        self.create_entries()
    }

    /// Resets desktop entries entirely by deleting the folder where they're stored and creating
    /// new entries for all games
    pub fn reset_entries(&self) -> Result<(), io::Error> {
        self.delete_all_entries()?;
        self.ensure_dir_desktop_files()?;
        self.create_entries()
    }

    /// Delete desktop entries folder and all it's contents
    pub fn delete_all_entries(&self) -> Result<(), io::Error> {
        if self.dir_desktop_files.is_dir() {
            remove_dir_all(&self.dir_desktop_files)?;
        };
        Ok(())
    }

    /// Deletes entries which don't have a matching game associated with them
    pub fn delete_dead_entries(&self) -> Result<(), io::Error> {
        let all_desktop_files = read_dir(&self.dir_desktop_files)?;

        let mut game_ids = HashSet::new();
        for game in &self.games {
            game_ids.insert(&game.appid);
        }

        for desktop_file in all_desktop_files.flatten() {
            let game_id = match desktop_file.file_name().to_str() {
                Some(filename) => filename.replace(".desktop", ""),
                _ => unreachable!(),
            };

            if !game_ids.contains(&game_id) {
                remove_file(desktop_file.path())?;
            };
        }

        Ok(())
    }

    /// Ensure a desktop entry exists for every game
    fn create_entries(&self) -> Result<(), io::Error> {
        for game in &self.games {
            // Path to desktop entry that will be created for this game
            let path_entry = self
                .dir_desktop_files
                .join(format!("{}.desktop", game.appid));

            // Skip if desktop entry already exists for this game
            if path_entry.is_file() {
                continue;
            }

            // Path to box art image to be used for desktop entry
            let path_box_art = self.dir_steam.join(format!(
                "appcache/librarycache/{}_library_600x900.jpg",
                &game.appid
            ));

            // Skip "games" lacking box art, which will include things like Proton and other things
            // that aren't necessarily games
            if !path_box_art.is_file() {
                continue;
            }

            // Skip if title could not be found in it's app manifest file
            let Some(title) = game.get_title()? else {
                continue;
            };

            self.create_entry(path_entry, path_box_art, &game.appid, title)?;
        }

        Ok(())
    }

    /// Create a single desktop entry
    fn create_entry(
        &self,
        path_entry: PathBuf,
        path_box_art: PathBuf,
        appid: &String,
        title: String,
    ) -> Result<(), io::Error> {
        let icon_string = get_str_from_path_buf(&path_box_art);

        let entry_contents = format!(
            "[Desktop Entry]\
            \nName={title}\
            \nExec=steam steam://rungameid/{appid}\
            \nIcon={icon_string}\
            \nTerminal=false\
            \nType=Application\
            \nCategories=RofiGames;"
        );

        let mut file = File::create(path_entry)?;
        file.write_all(entry_contents.as_bytes())?;

        Ok(())
    }

    /// Ensure folder for desktop entries is available
    fn ensure_dir_desktop_files(&self) -> Result<(), io::Error> {
        if !self.dir_desktop_files.is_dir() {
            create_dir(&self.dir_desktop_files)?;
        }
        Ok(())
    }
}
