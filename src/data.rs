use std::{
    borrow::Cow,
    ops::{Deref, DerefMut},
};

use lib_game_detector::data::{Game, SupportedLaunchers};
use redb::{Database, ReadableTable};
use tracing::trace;

use crate::db::{AccessData, TABLE};

#[derive(Debug)]
pub struct GameWithData {
    /// Game data.
    pub game: Game,
    /// Game access data, used for sorting.
    pub access_data: AccessData,
    /// Marks whether the game is sourced from a custom entry (user config).
    pub is_custom: bool,
}

impl GameWithData {
    pub fn from_game(game: Game, is_custom: bool) -> Self {
        GameWithData {
            game,
            is_custom,
            access_data: AccessData::default(),
        }
    }

    /// Get the entry's title in a format that is displayable within Pango markup.
    pub fn get_display_title<'a>(&'a self) -> Cow<'a, str> {
        if !self.title.contains('>')
            & !self.title.contains('<')
            & !self.title.contains('&')
            & !self.title.contains('\'')
        {
            return self.title.as_str().into();
        }

        // Escape characters
        let title = self.title.to_owned();
        let title = title.replace('>', "&gt;");
        let title = title.replace('<', "&lt;");
        let title = title.replace('&', "&amp;");
        let title = title.replace('\'', "&#39;");
        title.into()
    }

    /// Get the entry's source in a custom displayable format.
    pub fn get_display_source(&self) -> &'static str {
        use SupportedLaunchers::*;

        if self.is_custom {
            return "Custom";
        }

        match self.game.source {
            Steam | SteamShortcuts => "Steam",
            Lutris => "Lutris",
            Bottles => "Bottles",
            HeroicGamesAmazon | HeroicGamesEpic | HeroicGamesGOG | HeroicGamesSideload => {
                "Heroic Games Launcher"
            }
            MinecraftPrism => "Prism Launcher",
            MinecraftAT => "ATLauncher",
        }
    }
}

impl Deref for GameWithData {
    type Target = Game;
    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

impl DerefMut for GameWithData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.game
    }
}

pub fn wrap_games(games: Vec<Game>, db: &Database) -> Result<Vec<GameWithData>, redb::Error> {
    trace!("wrapping games with data from the DB");

    let write_txn = db.begin_write()?;
    let table = write_txn.open_table(TABLE)?;

    Ok(games
        .into_iter()
        .map(|game| {
            let mut wrapper = GameWithData::from_game(game, false);

            if let Ok(Some(e)) = table.get(wrapper.game.title.as_str()) {
                let val = e.value();
                wrapper.access_data.last_accessed = val.last_accessed;
                wrapper.access_data.count_accessed = val.count_accessed;
            };

            wrapper
        })
        .collect())
}
