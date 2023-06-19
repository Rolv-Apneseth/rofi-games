use log::{debug, error, warn};
use std::{
    collections::VecDeque,
    fs::{read_dir, File},
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use crate::helpers::get_regex;

use super::{Game, Launcher};

pub struct Steam {
    path_steam_dir: PathBuf,
}

impl Steam {
    pub fn new(path_steam_dir: PathBuf) -> Self {
        Steam { path_steam_dir }
    }
    /// Get all available steam libraries by parsing the libraryfolders.vdf file
    pub fn get_steam_libraries(&self) -> Result<Vec<SteamLibrary>, io::Error> {
        let mut libraries = Vec::new();

        let libraries_vdg_path = self
            .path_steam_dir
            .join("steamapps")
            .join("libraryfolders.vdf");

        debug!("Steam libraryfolders.vdf path: {libraries_vdg_path:?}");

        let libraries_vdg = File::open(libraries_vdg_path)?;

        let library_regex = get_regex("(\")/(.*)(\")");

        for line in BufReader::new(libraries_vdg).lines().flatten() {
            for capture in library_regex.captures_iter(&line) {
                let library_path = PathBuf::from(capture[0].replace('"', ""));
                libraries.push(SteamLibrary::new(
                    library_path,
                    self.path_steam_dir.to_owned(),
                ))
            }
        }

        if libraries.is_empty() {
            warn!("No steam libraries found")
        };

        Ok(libraries)
    }
}

impl Launcher for Steam {
    /// Get all available steam games
    fn get_games(&self) -> Result<Vec<Game>, ()> {
        let mut steam_games = Vec::new();

        let libraries = match self.get_steam_libraries() {
            Ok(l) => l,
            Err(e) => {
                error!("Error with parsing steam libraries:\n{e}");
                return Err(());
            }
        };

        for library in libraries {
            let mut games = match library.get_all_games() {
                Ok(g) => g,
                Err(e) => {
                    error!(
                        "Error with parsing games from a steam library.\nLibrary:
                        {library:?}\nError: {e:?}"
                    );
                    return Err(());
                }
            };

            steam_games.append(&mut games);
        }

        if steam_games.is_empty() {
            warn!("No games found for any steam library")
        }

        Ok(steam_games)
    }
}

// STEAM LIBRARY ------------------------------------------------------------------------
#[derive(Debug)]
pub struct SteamLibrary {
    path_library: PathBuf,
    path_steam_dir: PathBuf,
}
impl SteamLibrary {
    /// Returns a new SteamLibrary with the given path
    fn new(path_library: PathBuf, path_steam_dir: PathBuf) -> SteamLibrary {
        SteamLibrary {
            path_library,
            path_steam_dir,
        }
    }

    /// Find and return paths of the app manifest files, if they exist
    fn get_manifest_paths(&self) -> Result<Vec<PathBuf>, io::Error> {
        let mut manifest_paths = Vec::new();

        let manifest_regex = get_regex("appmanifest_.*.acf$");

        let paths = read_dir(&self.path_library.join("steamapps"))?;

        for path in paths.flatten() {
            let path_buf = path.path();

            if let Some(filename) = path_buf.to_str() {
                if manifest_regex.is_match(filename) {
                    manifest_paths.push(path_buf);
                };
            }
        }

        if manifest_paths.is_empty() {
            warn!(
                "No app manifest files found for steam library: {:?}",
                self.path_library
            );
        };

        Ok(manifest_paths)
    }

    /// Get all steam games associated with this library
    pub fn get_all_games(&self) -> Result<Vec<Game>, io::Error> {
        let mut games = Vec::new();

        for path_app_manifest in self.get_manifest_paths()? {
            if let Some(game) = self.get_game(path_app_manifest) {
                games.push(game)
            }
        }

        Ok(games)
    }

    /// Returns a new Game from the given path to a steam app manifest file (appmanifest_.*.acf)
    fn get_game(&self, path_app_manifest: PathBuf) -> Option<Game> {
        let appid = path_app_manifest
            .file_name()?
            .to_str()?
            .replace("appmanifest_", "")
            .replace(".acf", "");

        let title = self.parse_game_title(path_app_manifest)?;
        let launch_command = format!("steam steam://rungameid/{appid}");
        let path_icon = self
            .path_steam_dir
            .join(format!("appcache/librarycache/{appid}_library_600x900.jpg"));

        // Skip games withoug box art
        if !path_icon.is_file() {
            debug!("Skipped steam title as no box art exists for it: {title}");
            return None;
        }

        Some(Game {
            title,
            launch_command,
            path_icon,
        })
    }

    /// Parse game's title from the app manifest file
    fn parse_game_title(&self, path_app_manifest: PathBuf) -> Option<String> {
        let manifest_file = match File::open(&path_app_manifest) {
            Ok(t) => t,
            Err(e) => {
                error!("Error with reading app manifest file at {path_app_manifest:?}:\n{e}");
                return None;
            }
        };

        for line in BufReader::new(manifest_file).lines().flatten() {
            if line.contains("\"name\"") {
                let mut split_line = line.split_whitespace().collect::<VecDeque<&str>>();
                VecDeque::pop_front(&mut split_line);

                return Some(
                    split_line
                        .into_iter()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace(['\"', '™', '®'], ""),
                );
            }
        }

        debug!("No title could be parsed from app manifest file at: {path_app_manifest:?}");
        None
    }
}
