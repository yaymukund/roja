mod initialize_database;
mod metadata;
pub mod query;

use rusqlite::{Connection, OpenFlags};

pub use initialize_database::*;
pub use metadata::{FolderMetadata, TrackMetadata};

use crate::SETTINGS;

fn create_connection() -> Connection {
    let db_path = SETTINGS.with(|s| s.place_db_file());
    Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_CREATE,
    )
    .expect("could not open a read-only db connection")
}

thread_local! {
    pub static CONNECTION: Connection = create_connection();
}
