use std::{
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

/// Current Unix timestamp in seconds - based on system time.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("problem with system clock")
        .as_secs()
}

/// Expand tildes in user-given paths.
pub fn expand_tilde<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();

    if !path.starts_with("~") {
        return Some(path.to_path_buf());
    }

    let home = dirs::home_dir()?;
    if path == Path::new("~") {
        return Some(home);
    }

    let path = path.strip_prefix("~/").ok()?;

    if home == Path::new("/") {
        Some(path.to_path_buf())
    } else {
        Some(home.join(path))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let home = std::env::var("HOME").unwrap();
        let games = PathBuf::from(format!("{}/Games", home));
        assert_eq!(expand_tilde("~/Games"), Some(games));
        assert_eq!(
            expand_tilde("/absolute/path"),
            Some("/absolute/path".into())
        );
        assert_eq!(
            expand_tilde("~tester/projects"),
            Some("~tester/projects".into())
        );
    }
}
