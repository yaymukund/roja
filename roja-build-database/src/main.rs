#![warn(rust_2018_idioms)]
mod connection;
mod metadata;

use anyhow::{anyhow, Result};
use gumdrop::Options;
use log::{info, warn};
use rusqlite::named_params;
use walkdir::{DirEntry, WalkDir};

use std::ffi;
use std::path::{Path, PathBuf};

use connection::Connection;
use metadata::{FolderMetadata, TrackMetadata};

/// build the roja music database
#[derive(Options)]
struct CliOpts {
    /// directory containing mp3s
    #[options(required, short = "d")]
    pub root_directory: PathBuf,

    /// output filename
    #[options(required, short = "o")]
    pub out_file: PathBuf,

    /// if db doesn't exist, create it
    #[options(short = "c")]
    pub create: bool,

    #[options()]
    pub help: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = CliOpts::parse_args_default_or_exit();
    build_database(&cli.root_directory, &cli.out_file, cli.create)
}

pub fn build_database<P, Q>(root: P, outfile: Q, create: bool) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let conn = Connection::open(outfile)?;

    if create {
        conn.create_tables()?;
    }

    for dir_entry in child_dir_entries(&root) {
        if let Err(err) = process_entry(&conn, &dir_entry, &root) {
            warn!("error processing entry {}", err);
        }
    }

    Ok(())
}

fn process_entry<P>(conn: &Connection, dir_entry: &DirEntry, root: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = dir_entry.path();
    let relative_path = strip_prefix(path, &root)?;

    if conn.folder_exists(relative_path)? {
        info!("skipping path {}", relative_path);
        return Ok(());
    }

    if is_mp3(dir_entry) {
        let folder = FolderMetadata::load(path, relative_path)?;
        let folder_id = insert_folder(conn, &folder)?;
        let metadata = TrackMetadata::load(path, relative_path)?;
        insert_track(conn, &metadata, folder_id)?;
    } else {
        let tracks: Vec<TrackMetadata> = mp3_dir_entries(path)
            .filter_map(|d| {
                let path = d.path();
                let relative_path = strip_prefix(path, &root).ok()?;
                TrackMetadata::load(path, relative_path).ok()
            })
            .collect();

        if !tracks.is_empty() {
            let folder = FolderMetadata::load(&path, relative_path)?;
            let folder_id = insert_folder(conn, &folder)?;

            for track in tracks {
                insert_track(conn, &track, folder_id)?;
            }
        }
    }

    Ok(())
}

fn insert_folder(conn: &Connection, metadata: &FolderMetadata) -> Result<i64> {
    conn.insert_folder(named_params! {
        ":path": metadata.relative_path(),
        ":created_at": metadata.created_at(),
    })
}

fn insert_track(conn: &Connection, metadata: &TrackMetadata, folder_id: i64) -> Result<i64> {
    conn.insert_track(named_params! {
        ":title": metadata.title(),
        ":album": metadata.album(),
        ":artist": metadata.artist(),
        ":date": metadata.date(),
        ":track_number": metadata.track_number(),
        ":duration_seconds": metadata.duration_seconds(),
        ":path": metadata.relative_path(),
        ":folder_id": folder_id,
    })
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

fn strip_prefix<'a, P>(target: &'a Path, root: P) -> Result<&'a str>
where
    P: AsRef<Path>,
{
    target
        .strip_prefix(root)?
        .to_str()
        .ok_or(anyhow!("could not convert path to string: {:?}", target))
}
