use anyhow::Result;
use rusqlite::{named_params, Connection as RusqliteConnection, OptionalExtension, ToSql};

use std::path::Path;

pub struct Connection {
    conn: RusqliteConnection,
}

impl Connection {
    pub fn open<P>(db_path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let conn = RusqliteConnection::open(db_path)?;
        Ok(Self { conn })
    }

    pub fn folder_exists(&self, folder_path: &str) -> Result<bool> {
        let params = named_params! { ":path": folder_path };
        Ok(self.select_folder_id_where_path_eq(params)?.is_some())
    }

    pub fn select_folder_id_where_path_eq(
        &self,
        params: &[(&str, &dyn ToSql)],
    ) -> Result<Option<i64>> {
        Ok(self
            .conn
            .query_row_named(
                "SELECT id FROM folders WHERE path = :path LIMIT 1",
                params,
                |r| r.get(0),
            )
            .optional()?)
    }

    pub fn insert_folder<'a>(&self, params: &[(&str, &dyn ToSql)]) -> Result<i64> {
        self.conn.execute_named(
            "INSERT INTO folders (path, created_at) VALUES (:path, :created_at)",
            params,
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn insert_track<'a>(&self, params: &[(&str, &dyn ToSql)]) -> Result<i64> {
        self.conn.execute_named(
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

        Ok(self.conn.last_insert_rowid())
    }

    pub fn create_tables(&self) -> Result<()> {
        Ok(self.conn.execute_batch(
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
}
