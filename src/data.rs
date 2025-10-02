use std::ops::{Deref, DerefMut};

use lib_game_detector::data::Game;
use redb::{Database, ReadableTable};
use tracing::trace;

use crate::db::{AccessData, TABLE};

#[derive(Debug)]
pub struct GameWithData {
    /// Game data.
    pub game: Game,
    /// Game access data, used for sorting.
    pub access_data: AccessData,
}

impl GameWithData {
    pub fn from_game(inner: Game) -> Self {
        GameWithData {
            game: inner,
            access_data: AccessData::default(),
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
            let mut wrapper = GameWithData::from_game(game);

            if let Ok(Some(e)) = table.get(wrapper.game.title.as_str()) {
                let val = e.value();
                wrapper.access_data.last_accessed = val.last_accessed;
                wrapper.access_data.count_accessed = val.count_accessed;
            };

            wrapper
        })
        .collect())
}
