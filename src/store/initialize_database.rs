use rusqlite::{Connection, OpenFlags};

use crate::SETTINGS;

fn create_connection() -> Connection {
    let db_path = SETTINGS.with(|s| s.place_db_file());
    Connection::open_with_flags(db_path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .expect("could not open a read-only db connection")
}

thread_local! {
    static CONNECTION: Connection = create_connection();
}


