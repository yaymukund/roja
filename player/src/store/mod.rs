mod models;

use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use rusqlite::{named_params, Connection as RusqliteConnection, Row, NO_PARAMS};

use crate::SETTINGS;
pub use models::{Folder, Playlist, Track};

thread_local! {
    static CONN: Rc<RusqliteConnection> = initialize_db();
}

fn get_connection() -> Rc<RusqliteConnection> {
    CONN.with(|c| c.clone())
}

fn initialize_db() -> Rc<RusqliteConnection> {
    let db_path = SETTINGS.with(|s| s.place_db_file());
    let conn = RusqliteConnection::open(db_path).expect("could not open db");
    Rc::new(conn)
}

pub fn get_folders() -> Result<Vec<Folder>> {
    let conn = get_connection();
    let mut stmt = conn
        .prepare(
            "SELECT
                id,
                created_at,
                path
            FROM folders
            ORDER BY created_at DESC;",
        )
        .expect("couldn't execute query");

    let folders = stmt.query_map(NO_PARAMS, |row| {
        let id: i64 = row.get(0)?;
        let created_at: i64 = row.get(1)?;
        let path: String = row.get(2)?;

        Ok(Folder {
            id: id as usize,
            created_at: created_at as usize,
            path: PathBuf::from(path),
        })
    })?;

    let folders: Vec<Folder> = folders.filter_map(Result::ok).collect();
    log::info!("Found {} folders in db", folders.len());
    Ok(folders)
}

pub fn get_tracks_by_folder_id(folder_id: usize) -> Result<Vec<Track>> {
    let conn = get_connection();
    let mut stmt = conn.prepare(
        "SELECT
            id,
            title,
            album,
            artist,
            date,
            track_number,
            duration_seconds,
            path,
            folder_id
        FROM tracks
        WHERE folder_id = :folder_id",
    )?;

    let mut tracks: Vec<Track> = stmt
        .query_map_named(
            named_params! { ":folder_id": folder_id  as i64},
            row_to_track,
        )?
        .filter_map(Result::ok)
        .collect();

    tracks.sort_by(|t1, t2| alphanumeric_sort::compare_str(&t1.track_number, &t2.track_number));

    Ok(tracks)
}

pub fn get_tracks() -> Result<Vec<Track>> {
    let conn = get_connection();
    let mut stmt = conn.prepare(
        "SELECT
            id,
            title,
            album,
            artist,
            date,
            track_number,
            duration_seconds,
            path,
            folder_id
        FROM tracks;",
    )?;

    let tracks: Vec<Track> = stmt
        .query_map(NO_PARAMS, row_to_track)?
        .filter_map(Result::ok)
        .collect();

    log::info!("Found {} tracks in db", tracks.len());
    Ok(tracks)
}

pub fn get_paths_by_ids(ids: &[usize]) -> Result<Vec<PathBuf>> {
    let conn = get_connection();
    let params = (0..ids.len()).map(|_| "?").collect::<Vec<&str>>().join(",");
    let order_params = (0..ids.len())
        .map(|idx| format!("WHEN ? THEN {}", idx))
        .collect::<Vec<String>>()
        .join("\n");
    let mut ids: Vec<i64> = ids.iter().map(|id| *id as i64).collect();
    ids.extend(ids.clone());

    let mut stmt = conn.prepare(&format!(
        "SELECT
            id,
            path
        FROM tracks
        WHERE id IN ( {} )
        ORDER BY
            CASE id
            {}
            END;",
        params, order_params
    ))?;

    let paths: Vec<PathBuf> = stmt
        .query_map(&ids, |row| {
            let path_str: String = row.get(1)?;
            Ok(PathBuf::from(path_str))
        })?
        .filter_map(Result::ok)
        .collect();

    Ok(paths)
}

fn row_to_track(row: &Row<'_>) -> rusqlite::Result<Track> {
    let id: i64 = row.get(0)?;
    let duration: i64 = row.get(6)?;
    let path: String = row.get(7)?;
    let folder_id: i64 = row.get(8)?;

    Ok(Track {
        id: id as usize,
        title: row.get(1)?,
        album: row.get(2)?,
        artist: row.get(3)?,
        date: row.get(4)?,
        track_number: row.get(5)?,
        duration: duration as usize,
        path: PathBuf::from(path),
        folder_id: folder_id as usize,
    })
}
