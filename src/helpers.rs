use std::{path::PathBuf, process::exit};

use regex::Regex;

/// Returns a regex from a given pattern, exits if invalid
pub fn get_regex(pattern: &str) -> Regex {
    match Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error with creating regex: \n{}", e);
            exit(1);
        }
    }
}

/// Returns str representation of a PathBuf, exits if there is an error
pub fn get_str_from_path_buf(path_buf: &PathBuf) -> &str {
    match path_buf.as_os_str().to_str() {
        Some(s) => s,
        None => {
            eprintln!("Error with reading path as str from: {:?}", path_buf);
            exit(1);
        }
    }
}
