use std::path::PathBuf;

pub mod heroic;
pub mod lutris;
pub mod steam;

/// Struct which contains all the details necessary for a rofi entry (name, icon, command to run
/// when item is selected)
#[derive(Debug)]
pub struct Game {
    pub title: String,
    pub launch_command: String,
    pub path_icon: PathBuf,
}
impl Game {
    fn new(title: String, launch_command: String, path_icon: PathBuf) -> Self {
        Game {
            title,
            launch_command,
            path_icon,
        }
    }
}

pub trait Launcher {
    fn get_games(&self) -> Result<Vec<Game>, ()>;
}
