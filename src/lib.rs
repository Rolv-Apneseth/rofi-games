mod config;
mod data;
mod db;
mod utils;

use config::read_config;
use is_terminal::IsTerminal;
use lib_game_detector::get_detector;
use redb::Database;
use rofi_mode::{Action, Event};
use std::process::{self, Command};
use tracing::{debug, error, warn};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::{
    data::{GameWithData, wrap_games},
    db::{bump_entry, delete_entry, init_db},
};

struct Mode<'rofi> {
    entries: Vec<GameWithData>,
    api: rofi_mode::Api<'rofi>,
    db: Database,
}

// UTILS
impl Mode<'_> {
    /// Get a mutable reference to an entry from [`Self::entries`]
    ///
    /// # Panics
    /// Panics if the given selected index is out-of-bounds
    fn get_selected_entry(&mut self, selected: usize) -> &mut GameWithData {
        let selected = self
            .entries
            .get_mut(selected)
            .expect("Selected index is out-of-bounds");

        if let Err(e) = bump_entry(&self.db, selected) {
            error!("failed to bump access data DB entry: {e}");
        };

        selected
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

    /// Deletes access data for the selected game
    fn handle_delete_event(&mut self, selected: usize) {
        let title = { self.get_selected_entry(selected).title.clone() };
        match delete_entry(&self.db, title.as_str()) {
            Ok(Some(_)) => {}
            Ok(None) => warn!("could not find access data for entry {title}"),
            Err(e) => error!("failed to delete access data for entry {title}: {e}"),
        };
    }

    /// Get entries to be displayed by mode - games detected on system + custom entries.
    ///
    /// Exists in a separate method because the entries need to be re-generated in the case of
    /// the user deleting access data (sort order likely to change).
    fn get_entries(db: &Database) -> Result<Vec<GameWithData>, ()> {
        let games = get_detector().get_all_detected_games();
        let mut entries =
            wrap_games(games, db).map_err(|e| error!("failed to wrap games in inner type: {e}"))?;

        // Apply config, adding custom entries and sorting entries
        if let Some(config) = read_config() {
            config.apply(&mut entries);
        };

        Ok(entries)
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
            .with(
                fmt::layer()
                    .without_time()
                    .with_line_number(true)
                    .with_ansi(std::io::stdout().is_terminal()),
            )
            .with(EnvFilter::from_default_env())
            .init();

        let db = init_db().map_err(|e| error!("failed initialising DB: {e}"))?;
        let entries = Self::get_entries(&db)?;

        Ok(Mode { entries, api, db })
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
            // User accepted an entry from the list
            Event::Ok { alt, selected } => {
                if alt {
                    self.handle_alt_event_ok(selected)
                } else {
                    self.handle_regular_event_ok(selected)
                }
            }

            // User deleted an entry from the list
            Event::DeleteEntry { selected } => {
                self.handle_delete_event(selected);
                self.entries = Self::get_entries(&self.db).expect("failed resetting entries");
                return Action::Reset;
            }

            // User cancelled selection i.e. pressed `Esc`
            Event::Cancel { selected: _ } => {}

            // All other events currently unsupported
            _ => {
                error!("Unsupported input event: {event:?}")
            }
        }

        // Compact DB to save space (1.1M -> 53k on my machine™ with a couple entries)
        if let Err(e) = self.db.compact() {
            error!("failed to compact DB: {e}");
        };

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
