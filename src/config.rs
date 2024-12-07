use dirs::config_dir;
use lib_game_detector::data::Game;
use serde::Deserialize;
use std::{error::Error, fs::read_to_string, path::PathBuf, process::Command};
use tracing::{debug, error, trace, warn};

#[derive(Deserialize, Debug)]
pub struct Config {
    box_art_dir: Option<String>,
    entries: Vec<ConfigEntry>,
}

#[derive(Deserialize, Debug)]
pub struct ConfigEntry {
    title: String,
    launch_command: Option<String>,
    path_box_art: Option<String>,
    path_game_dir: Option<String>,
}

pub fn read_config() -> Option<Config> {
    let path_config = config_dir()?.join("rofi-games").join("config.toml");

    if !path_config.is_file() {
        debug!("Config file not found at {path_config:?}");
        return None;
    }

    debug!("Config file found at {:?}", &path_config);

    let Ok(contents) = read_to_string(path_config) else {
        error!("Could not read config file contents");
        return None;
    };

    toml::from_str::<Config>(&contents)
        .map_err(|e| {
            error!("Error parsing config: {:?}", e.message());
            if let Some(source) = e.source() {
                error!("Caused by: {source}");
            };
        })
        .ok()
}

/// Modify the given game entries with custom entries parsed from the config.
///
/// NOTE: entries are matched based on the title. Only the first game with the exact title
/// specified for the custom entry will be modified.
pub fn add_custom_entries(entries: &mut [Game], config: Config) {
    // Convert parsed config entries into a `Games` collection
    config.entries.into_iter().for_each(|entry| {
        let ConfigEntry {
            title,
            launch_command: opt_launch_command,
            path_box_art: opt_path_box_art,
            path_game_dir: opt_path_game_dir,
        } = entry;

        let Some(matching_entry) = entries.iter_mut().find(|e| e.title == title) else {
            return;
        };
        trace!("Matching entry for {title}: {matching_entry:?}");

        // REQUIRED FIELDS
        // Launch command
        if let Some(c) = opt_launch_command {
            if let Some(split_command) = shlex::split(&c) {
                let mut command = Command::new(&split_command[0]);
                command.args(&split_command[1..]);
                matching_entry.launch_command = command;
            } else {
                error!("Failed to split the given custom command: {c}");
            };
        }

        // Box art
        if let Some(p) = opt_path_box_art {
            let path = match config.box_art_dir.as_ref() {
                Some(d) => {
                    let path_dir = PathBuf::from(d);
                    if path_dir.is_absolute() {
                        path_dir.join(p)
                    } else {
                        warn!("Ignoring the given `box_art_dir` config option as it is not pointing to an absolute path");
                        PathBuf::from(p)
                    }
                }
                None => PathBuf::from(p),
            };

            if path.is_file() {
                matching_entry.path_box_art = Some(path);
            } else {
                error!("The box art path provided for '{title}' could not be found at: {path:?}");
            };
        } else {
            error!("No path to the box art provided for the custom entry with title: '{title}'");
        };

        // OPTIONAL FIELDS
        // Game directory
        if let Some(p) = opt_path_game_dir {
            let path = PathBuf::from(p);
            if path.is_dir() {
                matching_entry.path_game_dir = Some(path);
            } else {
                error!("The game directory path provided for '{title}' could not be found: {path:?}");
            };
        } else {
            debug!(
                "No path to the game directory provided for the custom entry with title: '{title}'"
            );
        };
    });
}

}
