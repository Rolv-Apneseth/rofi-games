use std::{
    collections::VecDeque,
    fs::{read_dir, File},
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::helpers::get_regex;

/// Get all available steam libraries by parsing the libraryfolders.vdf file
pub fn get_steam_libraries(steam_dir: &Path) -> Result<Vec<SteamLibrary>, io::Error> {
    let mut libraries = Vec::new();

    let libraries_vdg_path = steam_dir.join("steamapps").join("libraryfolders.vdf");
    let libraries_vdg = File::open(libraries_vdg_path)?;

    let library_regex = get_regex("(\")/(.*)(\")");

    for line in BufReader::new(libraries_vdg).lines().flatten() {
        for capture in library_regex.captures_iter(&line) {
            let library_path = PathBuf::from(capture[0].replace('"', ""));
            libraries.push(SteamLibrary::new(library_path))
        }
    }

    Ok(libraries)
}

// STEAM LIBRARY ------------------------------------------------------------------------
pub struct SteamLibrary {
    path: PathBuf,
}
impl SteamLibrary {
    /// Returns a new SteamLibrary with the given path
    fn new(path: PathBuf) -> SteamLibrary {
        SteamLibrary { path }
    }

    /// Find and return paths of the app manifest files, if they exist
    fn get_manifest_paths(&self) -> Result<Vec<PathBuf>, io::Error> {
        let mut manifest_paths = Vec::new();

        let manifest_regex = get_regex("appmanifest_.*.acf$");

        let paths = read_dir(&self.path.join("steamapps"))?;

        for path in paths.flatten() {
            let path_buf = path.path();

            if let Some(filename) = path_buf.to_str() {
                if manifest_regex.is_match(filename) {
                    manifest_paths.push(path_buf);
                };
            }
        }

        Ok(manifest_paths)
    }

    /// Get all steam games associated with this library
    pub fn get_games(&self) -> Result<Vec<SteamGame>, io::Error> {
        let mut games = Vec::new();

        for manifest_path in self.get_manifest_paths()? {
            if let Some(game) = SteamGame::new(manifest_path) {
                games.push(game);
            }
        }

        Ok(games)
    }
}

// STEAM GAME ---------------------------------------------------------------------------
pub struct SteamGame {
    path_app_manifest: PathBuf,
    pub appid: String,
}
impl SteamGame {
    /// Returns a new SteamGame from the given path to a steam app manifest file (appmanifest_.*.acf)
    fn new(path_app_manifest: PathBuf) -> Option<SteamGame> {
        let appid = path_app_manifest
            .file_name()?
            .to_str()?
            .replace("appmanifest_", "")
            .replace(".acf", "");

        Some(SteamGame {
            path_app_manifest,
            appid,
        })
    }

    /// Get game's title from the app manifest file
    pub fn get_title(&self) -> Result<Option<String>, io::Error> {
        let manifest_file = File::open(&self.path_app_manifest)?;

        for line in BufReader::new(manifest_file).lines().flatten() {
            if line.contains("\"name\"") {
                let mut split_line = line.split_whitespace().collect::<VecDeque<&str>>();
                VecDeque::pop_front(&mut split_line);

                return Ok(Some(
                    split_line
                        .into_iter()
                        .collect::<Vec<&str>>()
                        .join(" ")
                        .replace(['\"', '™', '®'], ""),
                ));
            }
        }

        Ok(None)
    }
}
