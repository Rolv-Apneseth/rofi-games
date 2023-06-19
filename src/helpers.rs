use log::{error, warn};
use std::{env, path::PathBuf, process::exit};

use regex::Regex;

/// Returns a regex from a given pattern, exits if invalid
pub fn get_regex(pattern: &str) -> Regex {
    match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            error!("Error with creating regex: \n{}", e);
            exit(1);
        }
    }
}

/// Returns str representation of a PathBuf, exits if OsStr is not valid UTF-8
pub fn get_str_from_path_buf(path_buf: &PathBuf) -> &str {
    match path_buf.as_os_str().to_str() {
        Some(s) => s,
        None => {
            error!("Error with reading path as str from: {:?}", path_buf);
            exit(1);
        }
    }
}

/// Gets an environment variable, or returns backup if there is an error
pub fn get_env(env: &str, backup: &str) -> String {
    match env::var(env) {
        Ok(e) => e,
        _ => {
            warn!("Could not find env variable ${env}, defaulting to backup: ${backup}");
            backup.to_owned()
        }
    }
}

/// Cleans up parsed game title
pub fn clean_game_title(title: &str) -> String {
    title.replace(['™', '®'], "")
}
