use config::read_config;
use lib_game_detector::{data::Game, get_detector};
use rofi_mode::{Action, Event};
use std::process::{self, Command};
use tracing::{debug, error};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use crate::config::add_custom_entries;

mod config;

struct Mode<'rofi> {
    entries: Vec<Game>,
    api: rofi_mode::Api<'rofi>,
}

// UTILS
impl<'rofi> Mode<'rofi> {
    /// Get a mutable reference to an entry from [`Self::entries`]
    ///
    /// # Panics
    /// Panics if the given selected index is out-of-bounds
    fn get_selected_entry(&mut self, selected: usize) -> &mut Game {
        self.entries
            .get_mut(selected)
            .expect("Selected index is out-of-bounds")
    }

    /// Attempts to launch selected game
    fn handle_regular_event_ok(&mut self, selected: usize) {
        let selected_entry = self.get_selected_entry(selected);
        if let Err(e) = selected_entry
            .launch_command
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .spawn()
        {
            error!("There was an error launching a game:\n{e}");
            debug!(
                "Launched with command:\n\t{:?}",
                selected_entry.launch_command
            );
        }
    }

    /// Attempts to open root directory of selected game
    fn handle_alt_event_ok(&mut self, selected: usize) {
        let selected_entry = self.get_selected_entry(selected);
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
        if api.display_name().is_none() {
            api.set_display_name("games");
        };

        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .init();

        let mut entries = get_detector().get_all_detected_games();

        // Add custom entries from config
        if let Some(config) = read_config() {
            add_custom_entries(&mut entries, config);
        };

        // Filter out entries without box art
        entries.retain(|e| e.path_box_art.is_some());

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
                match alt {
                    // User selected entry regularly, attempt to launch game
                    false => self.handle_regular_event_ok(selected),
                    // User selected entry with alternative binding, attempt to open game's root directory
                    true => self.handle_alt_event_ok(selected),
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

        self.api
            .query_icon(entry.path_box_art.as_ref()?.to_str()?, height)
            .wait(&mut self.api)
            .ok()
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
