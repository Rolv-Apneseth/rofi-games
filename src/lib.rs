use lib_game_detector::{data::GamesSlice, get_detector};
use log::{debug, error};
use std::process::{self, Command};

struct Mode<'rofi> {
    entries: GamesSlice,
    api: rofi_mode::Api<'rofi>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
    const NAME: &'static str = "games\0";

    fn init(mut api: rofi_mode::Api<'rofi>) -> Result<Self, ()> {
        api.set_display_name("Games");

        env_logger::init();

        let entries = get_detector()
            .get_all_detected_games_with_box_art()
            .ok_or_else(|| error!("Error getting games from detector."))?;

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
                error!("There was an error launching a game:\n{e}");
                debug!("Launched with command:\n\t{launch_command:?}")
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
            entry.path_box_art.as_ref()?.to_str()?,
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
