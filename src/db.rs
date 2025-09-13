use std::{fs::create_dir, io::Cursor, path::PathBuf};

use byteorder::{self, LittleEndian, ReadBytesExt};
use dirs::data_dir;
use redb::{Database, DatabaseError, TableDefinition, TypeName};
use tracing::{error, trace};

use crate::{data::GameWithData, utils::now};

pub const TABLE: TableDefinition<&str, AccessData> = TableDefinition::new("access_data");

#[derive(Debug, Default, Clone)]
pub struct AccessData {
    /// Unix timestamp (in seconds) representing the last time an entry was accessed.
    pub last_accessed: u64,
    /// The number of times the entry has been accessed.
    pub count_accessed: u32,
}

impl redb::Value for AccessData {
    type SelfType<'a> = AccessData;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        Some(size_of::<u64>() + size_of::<u32>())
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let mut data = Cursor::new(data);
        let last_accessed = data
            .read_u64::<LittleEndian>()
            .expect("invalid value stored for last_accessed");
        let count_accessed = data
            .read_u32::<LittleEndian>()
            .expect("invalid value stored for count_accessed");

        AccessData {
            last_accessed,
            count_accessed,
        }
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        value
            .last_accessed
            .to_le_bytes()
            .into_iter()
            .chain(value.count_accessed.to_le_bytes())
            .collect()
    }

    fn type_name() -> redb::TypeName {
        TypeName::new("game_access_data")
    }
}

/// Get the default path to the DB.
fn get_path_db() -> PathBuf {
    let parent = data_dir()
        .expect("could not get user data dir for system")
        .join("rofi-games");

    if !parent.is_dir() {
        let _ = create_dir(&parent).inspect_err(|e| error!("Could not create data dir: {e}"));
    }

    parent.join("access_data.db")
}

pub fn init_db() -> Result<redb::Database, DatabaseError> {
    let path_db = get_path_db();
    trace!("initialising DB at {path_db:?}");

    Database::create(path_db)
}

pub fn bump_entry(db: &mut Database, entry: &GameWithData) -> Result<(), redb::Error> {
    trace!("Bumping access data DB entry for '{}'", entry.title);

    let write_txn = db.begin_write()?;

    {
        let mut table = write_txn
            .open_table(TABLE)
            .inspect_err(|e| error!("failed to open DB table: {e}"))
            .unwrap();

        let _ = table.insert(
            entry.title.as_str(),
            AccessData {
                last_accessed: now(),
                count_accessed: entry.access_data.count_accessed + 1,
            },
        )?;
    };

    write_txn.commit()?;
    db.compact()?;

    Ok(())
}
