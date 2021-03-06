use anyhow::{anyhow, Result};
use rusqlite::named_params;
use walkdir::{DirEntry, WalkDir};

use std::ffi;
use std::path::{Path, PathBuf};

use crate::connection::Connection;
use crate::metadata::{FolderMetadata, TrackMetadata};
use crate::progress::Progress;

pub struct BuildDatabase<'a> {
    pub conn: &'a Connection,
    pub root: PathBuf,
    pub create: bool,
}

impl<'a> BuildDatabase<'a> {
    pub fn execute(&self) -> Result<()> {
        if self.create {
            self.conn.create_tables()?;
        }

        let dir_entries = self.directories();
        let mut progress = Progress::new(dir_entries.len());

        for dir_entry in dir_entries {
            match self.process_dir_entry(&dir_entry) {
                Err(err) => log::warn!("error processing entry {}", err),
                Ok(true) => progress.increment_added(),
                Ok(false) => progress.increment_skipped(),
            }
        }

        println!();
        Ok(())
    }

    fn process_dir_entry(&self, dir_entry: &DirEntry) -> Result<bool> {
        let path = dir_entry.path();
        let relative_path = strip_prefix(path, &self.root)?;

        if self.conn.folder_exists(relative_path)? {
            log::info!("skipping path {}", relative_path);
            return Ok(false);
        }

        if is_mp3(dir_entry) {
            let folder = FolderMetadata::load(path, relative_path)?;
            let folder_id = self.insert_folder(&folder)?;
            let metadata = TrackMetadata::load(path, relative_path)?;
            self.insert_track(&metadata, folder_id)?;
            Ok(true)
        } else {
            let tracks: Vec<TrackMetadata> = mp3s_in_path(path)
                .filter_map(|d| {
                    let path = d.path();
                    let relative_path = strip_prefix(path, &self.root).ok()?;
                    TrackMetadata::load(path, relative_path).ok()
                })
                .collect();

            if !tracks.is_empty() {
                let folder = FolderMetadata::load(&path, relative_path)?;
                let folder_id = self.insert_folder(&folder)?;

                for track in tracks {
                    self.insert_track(&track, folder_id)?;
                }
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }

    /// find all directories nested immediately under `path` (one level deep)
    fn directories(&self) -> Vec<DirEntry> {
        WalkDir::new(&self.root)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
            .collect()
    }

    fn insert_folder(&self, metadata: &FolderMetadata) -> Result<i64> {
        self.conn.insert_folder(named_params! {
            ":path": metadata.relative_path(),
            ":created_at": metadata.created_at(),
        })
    }

    fn insert_track(&self, metadata: &TrackMetadata, folder_id: i64) -> Result<i64> {
        self.conn.insert_track(named_params! {
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
}

/// find all mp3s nested under `path` (at any depth)
fn mp3s_in_path<P>(path: P) -> impl Iterator<Item = DirEntry>
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
