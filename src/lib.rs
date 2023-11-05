use lib_game_detector::{
    data::{Game, GamesSlice},
    get_detector,
};
use log::{debug, error};
use rofi_mode::{Action, Event};
use std::{
    process::{self, Command},
    sync::Arc,
};

struct Mode<'rofi> {
    entries: GamesSlice,
    api: rofi_mode::Api<'rofi>,
}

// UTILS
impl<'rofi> Mode<'rofi> {
    /// Attempts to launch selected game
    fn handle_regular_event_ok(&self, selected_entry: &Game) {
        let launch_command = selected_entry
            .launch_command
            .split_whitespace()
            .collect::<Arc<[&str]>>();

        if let Err(e) = Command::new(launch_command[0])
            .args(launch_command.iter().skip(1))
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .spawn()
        {
            error!("There was an error launching a game:\n{e}");
            debug!("Launched with command:\n\t{launch_command:?}");
        }
    }

    /// Attempts to open root directory of selected game
    fn handle_alt_event_ok(&self, selected_entry: &Game) {
        match selected_entry
            .path_game_dir
            .as_ref()
            .and_then(|d| d.to_str())
        {
            Some(game_dir) => {
                if let Err(e) = Command::new("xdg-open")
                    .arg(game_dir)
                    .stdout(process::Stdio::null())
                    .stderr(process::Stdio::null())
                    .spawn()
                {
                    error!("There was an error opening the directory to a game: {e:?}")
                }
            }
            None => {
                error!(
                    "Game directory for '({:?})' could not be found.",
                    selected_entry.title
                )
            }
        }
    }
}

// ROFI MODE
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
        match event {
            // User accepted an option from the list
            Event::Ok { alt, selected } => {
                let selected_entry = &self.entries[selected];

                match alt {
                    // User selected entry regularly, attempt to launch game
                    false => self.handle_regular_event_ok(selected_entry),
                    // User selected entry with alternative binding, attempt to open game's root directory
                    true => self.handle_alt_event_ok(selected_entry),
                }
            }

            // User cancelled selection i.e. pressed `Esc`
            Event::Cancel { selected: _ } => {}

            // All other events currently unsupported
            _ => {
                error!("Unsupported input event: {event:?}")
            }
        }

        Action::Exit
    }

    fn matches(&self, line: usize, matcher: rofi_mode::Matcher<'_>) -> bool {
        matcher.matches(&self.entries[line].title)
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
