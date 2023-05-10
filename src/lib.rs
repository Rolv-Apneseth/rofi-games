use games::{heroic::Heroic, steam::Steam, Game, Launcher};
use helpers::get_env;
use std::{
    path::PathBuf,
    process::{self, Command},
};

mod games;
mod helpers;

use crate::helpers::get_str_from_path_buf;

struct Mode<'rofi> {
    entries: Vec<Game>,
    api: rofi_mode::Api<'rofi>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "games\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("Games");

        let path_home = PathBuf::from(get_env("HOME", "~/"));
        let path_config_home = PathBuf::from(get_env("XDG_CONFIG_HOME", "~/.config"));

        let path_steam_dir = path_home.join(".local/share/Steam");
        let path_heroic_config = path_config_home.join("heroic");

        // Controller / manager for each supported launcher
        let steam = Steam::new(path_steam_dir);
        let heroic = Heroic::new(path_heroic_config);

        // Populate entries
        let mut entries = Vec::new();

        entries.append(&mut steam.get_games()?);
        entries.append(&mut heroic.get_games()?);

        Ok(Mode { entries, api })
    }

    fn entries(&mut self) -> usize {
        self.entries.len()
    }

    fn entry_content(&self, line: usize) -> rofi_mode::String {
        let entry = &self.entries[line];
        rofi_mode::format!("{}", entry.title)
    }

    fn react(
        &mut self,
        event: rofi_mode::Event,
        _input: &mut rofi_mode::String,
    ) -> rofi_mode::Action {
        if let rofi_mode::Event::Ok { alt: _, selected } = event {
            let selected_entry = &self.entries[selected];
            let launch_command = selected_entry
                .launch_command
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            if let Err(e) = Command::new(&launch_command[0])
                .args(launch_command.iter().skip(1))
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()
            {
                eprintln!("There was an error launching the game:\n{e}")
            };
        };

        rofi_mode::Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        let match_str = self.entries[line].title.as_str();
        matcher.matches(match_str)
    }

    fn entry_style(&self, _line: usize) -> rofi_mode::Style {
        rofi_mode::Style::default()
    }

    fn entry_attributes(&self, _line: usize) -> rofi_mode::Attributes {
        rofi_mode::Attributes::new()
    }

    fn entry_icon(&mut self, line: usize, height: u32) -> Option<rofi_mode::cairo::Surface> {
        let entry = &self.entries[line];
        rofi_mode::Api::query_icon(
            &mut self.api,
            get_str_from_path_buf(&entry.path_icon),
            height,
        )
        .wait(&mut self.api)
    }

    fn completed(&self, line: usize) -> rofi_mode::String {
        self.entry_content(line)
    }

    fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
        input.into()
    }

    fn message(&mut self) -> rofi_mode::String {
        rofi_mode::String::new()
    }
}

rofi_mode::export_mode!(Mode);
