use anyhow::{anyhow, Result};
use rusqlite::{named_params, Connection};
use walkdir::{DirEntry, WalkDir};

use log::{info, warn};
use std::ffi;
use std::path::Path;

use crate::store::{query, FolderMetadata, TrackMetadata};
use crate::SETTINGS;

fn create_connection() -> Connection {
    let db_path = SETTINGS.with(|s| s.place_db_file());
    Connection::open(db_path).expect("could not open a read/write db connection")
}

thread_local! {
    static CONNECTION: Connection = create_connection();
}

pub fn initialize_db<P>(load_path: P, create: bool)
where
    P: AsRef<Path>,
{
    if create {
        CONNECTION
            .with(query::create_tables)
            .expect("could not create tables");
    }

    for dir_entry in child_dir_entries(&load_path) {
        match process_entry(&dir_entry, &load_path) {
            Err(err) => warn!("error processing entry {}", err),
            _ => {}
        };
    }
}

fn process_entry<P>(dir_entry: &DirEntry, load_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = dir_entry.path();
    let relative_path = strip_prefix(path, &load_path)?;

    if folder_exists_in_db(relative_path)? {
        info!("skipping path {}", relative_path);
        return Ok(());
    }

    if is_mp3(dir_entry) {
        let folder = FolderMetadata::load(path, relative_path)?;
        let folder_id = insert_folder(&folder)?;
        let metadata = TrackMetadata::load(path, relative_path)?;
        insert_track(&metadata, folder_id)?;
    } else {
        let tracks: Vec<TrackMetadata> = mp3_dir_entries(path)
            .filter_map(|d| {
                let path = d.path();
                let relative_path = strip_prefix(path, &load_path).ok()?;
                TrackMetadata::load(path, relative_path).ok()
            })
            .collect();

        if !tracks.is_empty() {
            let folder = FolderMetadata::load(&path, relative_path)?;
            let folder_id = insert_folder(&folder)?;

            for track in tracks {
                insert_track(&track, folder_id)?;
            }
        }
    }

    Ok(())
}

fn folder_exists_in_db(relative_path: &str) -> Result<bool> {
    Ok(CONNECTION
        .with(|c| {
            let params = named_params! { ":path": relative_path };
            query::select_folder_id_where_path_eq(c, params)
        })?
        .is_some())
}

/// find all directories nested immediately under `path` (one level deep)
fn child_dir_entries<P>(path: P) -> impl Iterator<Item = DirEntry>
where
    P: AsRef<Path>,
{
    WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(Result::ok)
}

/// find all mp3s nested under `path` (at any depth)
fn mp3_dir_entries<P>(path: P) -> impl Iterator<Item = DirEntry>
where
    P: AsRef<Path>,
{
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(is_mp3)
}

fn is_mp3(dir_entry: &DirEntry) -> bool {
    dir_entry.path().extension().and_then(ffi::OsStr::to_str) == Some("mp3")
}

fn insert_folder(metadata: &FolderMetadata) -> Result<i64> {
    CONNECTION.with(|c| {
        query::insert_folder(
            c,
            named_params! {
                ":path": metadata.relative_path(),
                ":created_at": metadata.created_at(),
            },
        )
    })
}

fn insert_track(metadata: &TrackMetadata, folder_id: i64) -> Result<i64> {
    CONNECTION.with(|c| {
        query::insert_track(
            c,
            named_params! {
                ":title": metadata.title(),
                ":album": metadata.album(),
                ":artist": metadata.artist(),
                ":date": metadata.date(),
                ":track_number": metadata.track_number(),
                ":duration_seconds": metadata.duration_seconds(),
                ":path": metadata.relative_path(),
                ":folder_id": folder_id,
            },
        )
    })
}

fn strip_prefix<'a, P>(folder_path: &'a Path, load_path: P) -> Result<&'a str>
where
    P: AsRef<Path>,
{
    folder_path.strip_prefix(load_path)?.to_str().ok_or(anyhow!(
        "could not convert path to string: {:?}",
        folder_path
    ))
}
