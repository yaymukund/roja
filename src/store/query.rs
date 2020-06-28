use anyhow::Result;
use rusqlite::{Connection, OptionalExtension, ToSql};

pub fn select_folder_id_where_path_eq(
    conn: &Connection,
    params: &[(&str, &dyn ToSql)],
) -> Result<Option<i64>> {
    Ok(conn
        .query_row_named(
            "SELECT id FROM folders WHERE path = :path LIMIT 1",
            params,
            |r| r.get(0),
        )
        .optional()?)
}

pub fn insert_folder<'a>(conn: &Connection, params: &[(&str, &dyn ToSql)]) -> Result<i64> {
    conn.execute_named(
        "INSERT INTO folders (path, created_at) VALUES (:path, :created_at)",
        params,
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_track<'a>(conn: &Connection, params: &[(&str, &dyn ToSql)]) -> Result<i64> {
    conn.execute_named(
        "INSERT INTO tracks (
        title,
        album,
        artist,
        date,
        track_number,
        duration_seconds,
        path,
        folder_id
    ) VALUES (
        :title,
        :album,
        :artist,
        :date,
        :track_number,
        :duration_seconds,
        :path,
        :folder_id
    )",
        params,
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    Ok(conn.execute_batch(
        "
        BEGIN;
        CREATE TABLE folders(
            id INTEGER PRIMARY KEY,
            path TEXT NOT NULL UNIQUE,
            created_at DATETIME NOT NULL
        );

        CREATE TABLE tracks(
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            album TEXT NOT NULL,
            artist TEXT NOT NULL,
            date TEXT NOT NULL,
            track_number TEXT NOT NULL,
            duration_seconds INTEGER NOT NULL,
            path TEXT NOT NULL,
            folder_id INTEGER NOT NULL,
            FOREIGN KEY (folder_id)
                REFERENCES folders (id)
                    ON DELETE RESTRICT
                    ON UPDATE CASCADE
        );
        COMMIT;
        ",
    )?)
}
