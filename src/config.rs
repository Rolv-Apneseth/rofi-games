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

#[cfg(test)]
pub mod test_config {
    use std::{ops::Range, sync::LazyLock};
    use test_case::test_case;

    use super::*;

    const CMD: &str = "cmd";

    fn get_dummy_games() -> Vec<Game> {
        (1..11)
            .map(|i| Game {
                title: i.to_string(),
                path_box_art: Some(PathBuf::default()),
                path_game_dir: Some(PathBuf::default()),
                launch_command: Command::new(CMD),
            })
            .collect()
    }

    #[test_case(1..2; "single")]
    #[test_case(3..7; "multiple")]
    #[test_case(1..11; "all")]
    fn test_add_custom_entries_launch_command(range: Range<u16>) {
        let mut entries = get_dummy_games();

        let cmd = "new_command";

        add_custom_entries(
            &mut entries,
            Config {
                box_art_dir: None,
                entries: range
                    .clone()
                    .map(|i| ConfigEntry {
                        title: i.to_string(),
                        launch_command: Some(cmd.to_string()),
                        path_box_art: None,
                        path_game_dir: None,
                    })
                    .collect(),
            },
        );

        // Launch command changed, but nothing else
        range.for_each(|i| {
            let entry = entries.iter().find(|e| e.title == i.to_string()).unwrap();
            assert_eq!(entry.launch_command.get_program(), cmd);
            assert!(entry.path_game_dir.is_some(),);
            assert!(entry.path_box_art.is_some());
        });
    }

    #[test_case(3..4; "single")]
    #[test_case(4..8; "multiple")]
    #[test_case(1..11; "all")]
    fn test_add_custom_entries_game_dir(range: Range<u16>) {
        let mut entries = get_dummy_games();
        let config_entries = range
            .clone()
            .map(|i| ConfigEntry {
                title: i.to_string(),
                launch_command: None,
                path_box_art: None,
                path_game_dir: Some(PATH_PARENT_DIR.clone()),
            })
            .collect();

        add_custom_entries(
            &mut entries,
            Config {
                box_art_dir: None,
                entries: config_entries,
            },
        );

        // Path to the game dir changed, but nothing else
        range.for_each(|i| {
            let entry = entries.iter().find(|e| e.title == i.to_string()).unwrap();
            assert_eq!(
                entry.path_game_dir.clone().unwrap(),
                PathBuf::from(PATH_PARENT_DIR.as_str())
            );
            assert!(entry.path_box_art.is_some());
            assert_eq!(entry.launch_command.get_program(), CMD)
        });
    }

    static PATH_PARENT_DIR: LazyLock<String> = LazyLock::new(|| {
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(PathBuf::from(file!()).parent().unwrap())
            .to_string_lossy()
            .to_string()
    });

    #[test_case(1..2, None; "single")]
    #[test_case(1..2, Some(PATH_PARENT_DIR.clone()); "single with base dir")]
    #[test_case(3..7, None; "multiple")]
    #[test_case(3..7, Some(PATH_PARENT_DIR.clone()); "multiple with base dir")]
    #[test_case(1..11, None; "all")]
    #[test_case(1..11, Some(PATH_PARENT_DIR.clone()); "all with base dir")]
    fn test_add_custom_entries_box_art(range: Range<u16>, box_art_dir: Option<String>) {
        let mut entries = get_dummy_games();

        let path: String;
        let final_path_to_match: PathBuf;

        if box_art_dir.is_some() {
            path = file!().replace("src/", "");
            final_path_to_match = PathBuf::from(box_art_dir.clone().unwrap()).join(path.clone());
        } else {
            path = file!().to_string();
            final_path_to_match = PathBuf::from(path.clone());
        };

        add_custom_entries(
            &mut entries,
            Config {
                box_art_dir,
                entries: range
                    .clone()
                    .map(|i| ConfigEntry {
                        title: i.to_string(),
                        launch_command: None,
                        path_box_art: Some(path.clone()),
                        path_game_dir: None,
                    })
                    .collect(),
            },
        );

        // Path to the box art changed, but nothing else
        range.for_each(|i| {
            let entry = entries.iter().find(|e| e.title == i.to_string()).unwrap();
            assert_eq!(entry.path_box_art.as_ref().unwrap(), &final_path_to_match);
            assert!(entry.path_game_dir.is_some());
            assert_eq!(entry.launch_command.get_program(), CMD)
        });
    }
}
