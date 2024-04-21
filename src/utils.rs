use std::{
    path::PathBuf,
    process::Command,
    sync::{Arc, Mutex},
};

use lib_game_detector::data::Game;
use tracing::{error, warn};

#[inline]
pub fn get_launch_command(
    matching_entry: Option<&Game>,
    opt_launch_command: &Option<String>,
    title: &str,
) -> Option<Arc<Mutex<Command>>> {
    match (matching_entry, opt_launch_command) {
        (_, Some(c)) => {
            let split_command = shlex::split(c)?;
            let mut command = Command::new(&split_command[0]);
            command.args(&split_command[1..]);
            Some(Arc::new(Mutex::new(command)))
        }
        (Some(entry), None) => Some(entry.launch_command.clone()),
        _ => {
            error!("No launch command provided for the custom entry with title: '{title}'");
            None
        }
    }
}

#[inline]
pub fn get_path_box_art(
    matching_entry: Option<&Game>,
    opt_path_box_art: &Option<String>,
    opt_path_box_art_dir: &Option<String>,
    title: &str,
) -> Option<PathBuf> {
    match (matching_entry, opt_path_box_art) {
        (_, Some(p)) => {
            let path = match opt_path_box_art_dir {
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
                Some(path)
            } else {
                error!("The box art path provided for '{title}' could not be found at: {path:?}");
                None
            }
        }
        (Some(entry), None) => entry.path_box_art.clone(),
        _ => {
            error!("No path to the box art provided for the custom entry with title: '{title}'");
            None
        }
    }
}

#[inline]
pub fn get_path_game_dir(
    matching_entry: Option<&Game>,
    opt_path_game_dir: &Option<String>,
    title: &str,
) -> Option<PathBuf> {
    match (matching_entry, opt_path_game_dir) {
        (_, Some(p)) => {
            let path = PathBuf::from(p);
            if path.is_dir() {
                Some(path)
            } else {
                error!("The game directory path provided for '{title}' could not be found: {p}");
                None
            }
        }
        (Some(entry), None) => entry.path_game_dir.clone(),
        _ => {
            error!(
                "No path to the game directory provided for the custom entry with title: '{title}'"
            );
            None
        }
    }
}
