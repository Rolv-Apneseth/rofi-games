use log::{debug, trace, warn};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

use crate::helpers::{clean_game_title, parse_value_from_json_line};

use super::{Game, Launcher};

pub struct Heroic {
    path_heroic_config: PathBuf,
}

impl Heroic {
    pub fn new(path_config_home: &Path) -> Self {
        let path_heroic_config = path_config_home.join("heroic");
        debug!("Heroic dir path exists: {}", path_heroic_config.is_dir());

        Heroic { path_heroic_config }
    }

    /// Parses a given library file and returns a vec containing Game structs corresponding to the
    /// installed games
    pub fn parse_library(&self, path_library_json: PathBuf) -> Result<Vec<Game>, io::Error> {
        let mut games = Vec::new();

        let library_json = File::open(&path_library_json)?;

        // Assumptions made for below code to parse games correctly:
        // - Every game has defined: app_name, title, is_installed
        // - The "title" field for actual game is defined last out of the required fields
        let mut app_names = Vec::new();
        let mut are_installed = Vec::new();

        trace!("Parsing heroic launcher library json file at {path_library_json:?}");
        for line in BufReader::new(library_json).lines().flatten() {
            trace!(
                "app_names: {app_names:?}\nare_installed: {are_installed:?}\nCurrent line:
                {line}"
            );

            if line.contains("\"title\"")
                && are_installed.len() == app_names.len()
                && are_installed[are_installed.len() - 1]
            {
                let Some(mut title) = parse_value_from_json_line(&line) else {
                    continue;
                };
                title = clean_game_title(&title);

                let app_name = &app_names[app_names.len() - 1];

                // ADD GAME
                let launch_command = format!(
                    "xdg-open
heroic://launch/legendary/{app_name}"
                );

                let path_icon = self
                    .path_heroic_config
                    .join(format!("icons/{app_name}.jpg"));

                if path_icon.is_file() {
                    games.push(Game::new(title.to_owned(), launch_command, path_icon));
                };
            // IS_INSTALLED
            } else if line.contains("\"is_installed\"") {
                let Some(is_installed) = parse_value_from_json_line(&line) else {
                    continue;
                };

                are_installed.push(matches!(is_installed.as_str(), "true"));
            // APP_NAME
            } else if line.contains("\"app_name\"") {
                let Some(app_name) = line.split_once("\": \"")
                    .and_then(|split_line| split_line.1.strip_suffix("\","))
                else {
                    continue;
                };

                app_names.push(app_name.to_owned());
            }
        }

        if games.is_empty() {
            warn!("No games found for heroic launcher library at: {path_library_json:?}")
        }

        Ok(games)
    }

    /// Parses the gog_store/installed.json file returns a vec containing Game structs
    /// corresponding to the installed games
    /// This function is necessary and separate to parse_library because for some reason the
    /// `is_installed` value in the gog library file is not actually used by the program and hence
    /// always left false. Also there is no equivalent `installed` file for Legendary games
    /// This is confirmed in the following issue: https://github.com/Heroic-Games-Launcher/HeroicGamesLauncher/issues/2691#issuecomment-1540063754
    pub fn parse_gog_installed(
        &self,
        path_gog_installed_json: PathBuf,
    ) -> Result<Vec<Game>, io::Error> {
        let mut games = Vec::new();

        let installed_json = File::open(&path_gog_installed_json)?;

        let mut app_names = Vec::new();
        let mut titles = Vec::new();

        trace!(
            "Parsing heroic library file for installed GOG games at {path_gog_installed_json:?}"
        );
        for line in BufReader::new(installed_json).lines().flatten() {
            trace!(
                "app_names: {app_names:?}\ntitles: {titles:?}\nCurrent line:
                {line}"
            );

            if line.contains("\"appName\":") {
                let Some(app_name) = parse_value_from_json_line(&line) else {
                    continue;
                };

                app_names.push(app_name);
            } else if line.contains("\"install_path\":") {
                if let Some(title) = parse_value_from_json_line(&line).and_then(|install_path| {
                    install_path
                        .rsplit_once('/')
                        .map(|split_path| split_path.1.to_owned())
                }) {
                    titles.push(title)
                };
            };
        }

        // Loop over parsed app_names and titles
        app_names.into_iter().enumerate().for_each(|(i, app_name)| {
            let title = &titles[i];

            let launch_command = format!(
                "xdg-open
heroic://launch/legendary/{app_name}"
            );

            let path_icon = self
                .path_heroic_config
                // GOG games use png, Legendary use jpg
                .join(format!("icons/{app_name}.png"));

            if !path_icon.is_file() {
                warn!(
                    "Skipping GOG game with title {title} as the icon file does not exist at
{path_icon:?}"
                );
                return;
            }

            games.push(Game::new(title.to_owned(), launch_command, path_icon));
        });

        if games.is_empty() {
            warn!("No games found for Heroic launcher GOG library at {path_gog_installed_json:?}");
        }

        Ok(games)
    }
}

impl Launcher for Heroic {
    fn get_games(&self) -> Result<Vec<Game>, ()> {
        let mut games = Vec::new();

        // Legendary (Epic) games
        let path_library_legendary = self
            .path_heroic_config
            .join("store_cache/legendary_library.json");

        match self.parse_library(path_library_legendary) {
            Ok(mut legendary_games) => games.append(&mut legendary_games),
            Err(e) => warn!("Error with parsing heroic launcher legendary games library:\n{e}"),
        };

        // GOG games
        let path_gog_installed_json = self.path_heroic_config.join("gog_store/installed.json");
        match self.parse_gog_installed(path_gog_installed_json) {
            Ok(mut gog_games) => games.append(&mut gog_games),
            Err(e) => warn!("Error with parsing heroic launcher gog games library:\n{e}"),
        };

        Ok(games)
    }
}
