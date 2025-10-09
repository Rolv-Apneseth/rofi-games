use dirs::config_dir;
use lib_game_detector::data::{Game, SupportedLaunchers};
use serde::Deserialize;
use std::{cmp::Ordering, error::Error, fs::read_to_string, path::PathBuf, process::Command};
use tracing::{debug, error, trace, warn};

use crate::{data::GameWithData, utils::now};

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    hide_entries_without_box_art: Option<bool>,
    fallback_to_icons: Option<bool>,
    box_art_dir: Option<String>,
    entries: Vec<ConfigEntry>,

    sort: SortConfig,
    pub style: StyleConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct StyleConfig {
    pub show_entry_source_text: Option<bool>,
    pub use_bold_entry_title: Option<bool>,
}

#[derive(Deserialize, Debug, Default)]
struct SortConfig {
    order: Option<SortOrder>,
    reverse: Option<bool>,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    /// Sort games alphabetically by their title.
    Alphabetical,
    /// Sort games by recently have been accessed (through this program).
    Recency,
    /// Sort games by how often they have been accessed (through this program).
    Frequency,
    /// Sort games by a combination of frequency and recency.
    #[default]
    Frecency,
    /// Don't apply any sort order, leaving games in the order they were originally parsed.
    None,
}

#[derive(Deserialize, Debug, Default)]
struct ConfigEntry {
    title: String,
    launch_command: Option<String>,
    path_box_art: Option<String>,
    path_game_dir: Option<String>,
    hide: Option<bool>,
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

impl Config {
    /// Apply config options to the given entries.
    pub fn apply(&self, entries: &mut Vec<GameWithData>) {
        self.apply_custom_entries(entries);

        let hide_entries_without_box_art = self.hide_entries_without_box_art.unwrap_or(false);
        let fallback_to_icons = self.fallback_to_icons.unwrap_or(true);

        if fallback_to_icons {
            entries.iter_mut().for_each(|entry| {
                if entry.path_box_art.is_none() {
                    entry.path_box_art = entry.path_icon.take();
                }
            });
        }

        if hide_entries_without_box_art {
            entries.retain(|e| e.path_box_art.is_some());
        }

        self.sort_entries(entries);
    }

    /// Modify the given game entries with custom entries parsed from the config.
    ///
    /// NOTE: entries are matched based on the title. Only the first game with the exact title
    /// specified for the custom entry will be modified.
    fn apply_custom_entries(&self, entries: &mut Vec<GameWithData>) {
        // Convert parsed config entries into a `GameWrappers` collection
        self.entries.iter().for_each(|entry| {
            let ConfigEntry {
                title,
                launch_command: opt_launch_command,
                path_box_art: opt_path_box_art,
                path_game_dir: opt_path_game_dir,
                hide,
            } = entry;

            // HIDE ENTRY
            if hide.unwrap_or(false) {
                if let Some((index, _)) = entries.iter().enumerate().find(|(_, g)| g.title == *title) {
                    entries.swap_remove(index);
                } else {
                    warn!("Could not find an entry with title '{title}' to hide... skipping")
                };
                return;
            }

            let (mut opt_command, mut path_box_art, mut path_game_dir) = (None, None, None);

            // REQUIRED FIELDS
            // Launch command
            if let Some(c) = opt_launch_command {
                if let Some(split_command) = shlex::split(c) {
                    let mut command = Command::new(&split_command[0]);
                    command.args(&split_command[1..]);
                    opt_command = Some(command);
                } else {
                    error!("Failed to split the given custom command: {c}");
                    return;
                };
            }

            // Box art
            if let Some(p) = opt_path_box_art {
                let path = match self.box_art_dir.as_ref() {
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
                    path_box_art = Some(path);
                } else {
                    error!("The box art path provided for '{title}' could not be found at: {path:?}");
                    return;
                };
            };

            // OPTIONAL FIELDS
            // GameWrapper directory
            if let Some(p) = opt_path_game_dir {
                let path = PathBuf::from(p);
                if path.is_dir() {
                    path_game_dir = Some(path);
                } else {
                    error!("The game directory path provided for '{title}' could not be found: {path:?}");
                };
            } else {
                debug!(
                    "No path to the game directory provided for the custom entry with title: '{title}'"
                );
            };

            if let Some(matching_entry) = entries.iter_mut().find(|e| e.title == *title) {
                // MODIFY EXISTING ENTRY
                trace!("Matching entry for {title}: {:?}", matching_entry.game);

                    if let Some(launch_command) = opt_command {
                        matching_entry.launch_command = launch_command;
                    };

                    match (&matching_entry.path_box_art, path_box_art) {
                        (_, Some(p)) => matching_entry.path_box_art = Some(p),
                        (None, _) => error!("No path to the box art specified for entry with title: '{title}'"),
                        _ => {},
                    }

                    if let Some(p) = path_game_dir {
                        matching_entry.path_game_dir = Some(p);
                    }
            } else {
                // ADD FULLY CUSTOM ENTRY
                    trace!("Creating fully custom entry for {title}");

                    let Some(launch_command) = opt_command else {
                        error!("No launch command specified for entry with title: '{title}'");
                        return;
                    };

                    entries.push(GameWithData::from_game(Game {
                        title: title.clone(),
                        launch_command,
                        path_box_art,
                        path_game_dir,
                        path_icon: None,
                        // Steam used as a default
                        source: SupportedLaunchers::Steam,
                    }, true))
            };
        });
    }

    /// Sort entries.
    pub fn sort_entries(&self, entries: &mut [GameWithData]) {
        let sort_order = self.sort.order.clone().unwrap_or_default();
        let reverse = self.sort.reverse.unwrap_or_default();

        // NOTE: All sorting methods fallback to alphabetical sort for equal entries
        fn fallback_alphabetical(a: &GameWithData, b: &GameWithData) -> Ordering {
            a.game.title.to_lowercase().cmp(&b.title.to_lowercase())
        }

        /// Re-order arguments based on whether the order of elements should be reversed.
        ///
        /// i.e. a > b, but if reversing the order, b > a
        fn order_args<'a>(
            a: &'a GameWithData,
            b: &'a GameWithData,
            reverse: bool,
        ) -> (&'a GameWithData, &'a GameWithData) {
            if reverse { (b, a) } else { (a, b) }
        }

        match sort_order {
            SortOrder::Recency => {
                entries.sort_by(|a, b| {
                    let (a, b) = order_args(a, b, reverse);

                    match b
                        .access_data
                        .last_accessed
                        .cmp(&a.access_data.last_accessed)
                    {
                        Ordering::Equal => fallback_alphabetical(a, b),
                        other => other,
                    }
                });
            }
            SortOrder::Frequency => {
                entries.sort_by(|a, b| {
                    let (a, b) = order_args(a, b, reverse);

                    match b
                        .access_data
                        .count_accessed
                        .cmp(&a.access_data.count_accessed)
                    {
                        Ordering::Equal => fallback_alphabetical(a, b),
                        other => other,
                    }
                });
            }
            SortOrder::Frecency => {
                let now = now();

                entries.sort_by(|a, b| {
                    let (a, b) = order_args(a, b, reverse);

                    let get_score = |last_accessed: u64, count_accessed: u32| -> u64 {
                        // Skip weight calculations for values which have not been accessed
                        if last_accessed == 0 || count_accessed == 0 {
                            return 0;
                        }

                        let elapsed = now - last_accessed;
                        let elapsed_days = elapsed.saturating_div(60 * 60 * 24);
                        let weight_recency = match elapsed_days {
                            0 => 5000,
                            1 => 2500,
                            2 => 1000,
                            3 => 500,
                            d => 1000u64.saturating_div(d),
                        };
                        let weight_frequency = count_accessed as u64;

                        weight_frequency.saturating_add(weight_recency)
                    };

                    let af = get_score(a.access_data.last_accessed, a.access_data.count_accessed);
                    let bf = get_score(b.access_data.last_accessed, b.access_data.count_accessed);

                    match bf.cmp(&af) {
                        Ordering::Equal => fallback_alphabetical(a, b),
                        other => other,
                    }
                });
            }
            SortOrder::Alphabetical => {
                entries.sort_unstable_by_key(|e| e.title.to_lowercase());
                if reverse {
                    entries.reverse();
                }
            }
            SortOrder::None => {}
        };
    }
}

#[cfg(test)]
pub mod test_config {
    use lib_game_detector::data::Game;
    use std::{ops::Range, sync::LazyLock};
    use test_case::test_case;

    use crate::data::GameWithData;

    use super::*;

    const CMD: &str = "cmd";

    fn get_dummy_games() -> Vec<GameWithData> {
        (1..11)
            .map(|i| {
                GameWithData::from_game(
                    Game {
                        title: i.to_string(),
                        path_box_art: Some(PathBuf::default()),
                        path_game_dir: Some(PathBuf::default()),
                        launch_command: Command::new(CMD),
                        path_icon: Some(PathBuf::default()),
                        source: SupportedLaunchers::Steam,
                    },
                    false,
                )
            })
            .collect()
    }

    #[test_case(1..2; "single")]
    #[test_case(3..7; "multiple")]
    #[test_case(1..11; "all")]
    fn test_add_custom_entries_launch_command(range: Range<u16>) {
        let mut entries = get_dummy_games();

        let cmd = "new_command";

        Config {
            entries: range
                .clone()
                .map(|i| ConfigEntry {
                    title: i.to_string(),
                    launch_command: Some(cmd.to_owned()),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
        .apply(&mut entries);

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
                path_game_dir: Some(PATH_PARENT_DIR.clone()),
                ..Default::default()
            })
            .collect();

        Config {
            entries: config_entries,
            ..Default::default()
        }
        .apply(&mut entries);

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
            path = file!().to_owned();
            final_path_to_match = PathBuf::from(path.clone());
        };

        Config {
            box_art_dir,
            entries: range
                .clone()
                .map(|i| ConfigEntry {
                    title: i.to_string(),
                    path_box_art: Some(path.clone()),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
        .apply(&mut entries);

        // Path to the box art changed, but nothing else
        range.for_each(|i| {
            let entry = entries.iter().find(|e| e.title == i.to_string()).unwrap();
            assert_eq!(entry.path_box_art.as_ref().unwrap(), &final_path_to_match);
            assert!(entry.path_game_dir.is_some());
            assert_eq!(entry.launch_command.get_program(), CMD)
        });
    }

    #[test]
    fn test_add_fully_custom_entries() {
        let mut entries = get_dummy_games();
        let old_len = entries.len();
        let new_titles = ["a", "b"];

        Config {
            entries: new_titles
                .iter()
                .map(|title| ConfigEntry {
                    title: (*title).to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    path_box_art: Some(file!().to_owned()),
                    ..Default::default()
                })
                .collect(),
            ..Default::default()
        }
        .apply(&mut entries);

        assert_eq!(entries.len(), old_len + 2);
        assert_eq!(entries[10].title, new_titles[0]);
        assert_eq!(entries[11].title, new_titles[1]);
    }

    #[test]
    fn test_skips_faulty_fully_custom_entries() {
        let mut entries = get_dummy_games();
        let old_len = entries.len();
        let new_titles = ["a", "b", "c"];

        Config {
            entries: vec![
                ConfigEntry {
                    title: new_titles[0].to_owned(),
                    path_box_art: Some(file!().to_owned()),
                    ..Default::default()
                },
                ConfigEntry {
                    ..Default::default()
                },
                ConfigEntry {
                    title: new_titles[1].to_owned(),
                    ..Default::default()
                },
                ConfigEntry {
                    title: new_titles[2].to_owned(),
                    path_game_dir: Some(file!().to_owned()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
        .apply(&mut entries);

        assert_eq!(entries.len(), old_len);
    }

    #[test]
    fn test_skips_entries_without_box_art() {
        let mut entries = get_dummy_games();
        let old_len = entries.len();
        let new_titles = ["a", "b"];

        Config {
            entries: vec![
                // Should not get skipped
                ConfigEntry {
                    title: new_titles[0].to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    path_box_art: Some(file!().to_owned()),
                    ..Default::default()
                },
                // Should get skipped
                ConfigEntry {
                    title: new_titles[1].to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    ..Default::default()
                },
            ],
            hide_entries_without_box_art: Some(true),
            ..Default::default()
        }
        .apply(&mut entries);

        assert_eq!(entries.len(), old_len + 1);
        assert!(entries.iter().any(|e| e.title == new_titles[0]));
        assert!(!entries.iter().any(|e| e.title == new_titles[1]));
    }

    #[test]
    fn test_skips_hidden_entries() {
        let mut entries = get_dummy_games();
        let old_len = entries.len();
        let new_titles = ["a", "b", "c"];

        Config {
            entries: vec![
                // Should not get skipped
                ConfigEntry {
                    title: new_titles[0].to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    hide: Some(false),
                    ..Default::default()
                },
                ConfigEntry {
                    title: new_titles[1].to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    hide: None,
                    ..Default::default()
                },
                // Should get skipped
                ConfigEntry {
                    title: new_titles[2].to_owned(),
                    launch_command: Some(CMD.to_owned()),
                    hide: Some(true),
                    ..Default::default()
                },
                // Should get removed
                ConfigEntry {
                    title: entries[0].title.to_string(),
                    hide: Some(true),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
        .apply(&mut entries);

        // 2 new entries, 1 existing entry hidden
        assert_eq!(entries.len(), old_len + 2 - 1);
        assert!(entries.iter().any(|e| e.title == new_titles[0]));
        assert!(entries.iter().any(|e| e.title == new_titles[1]));
        assert!(!entries.iter().any(|e| e.title == new_titles[2]));
    }
}
