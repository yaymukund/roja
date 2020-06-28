use anyhow::{anyhow, Context, Result};
use filetime::FileTime;
use id3::{self, Tag};

use std::fmt::Debug;
use std::fs;
use std::path::Path;

pub struct TrackMetadata {
    tag: Tag,
    duration_seconds: i64,
    relative_path: String,
}

pub struct FolderMetadata {
    created_at: i64,
    relative_path: String,
}

impl FolderMetadata {
    pub fn load<P>(path: P, relative_path: &str) -> Result<Self>
    where
        P: Debug + AsRef<Path>,
    {
        let metadata = fs::metadata(&path)
            .with_context(|| format!("failed to read metadata of path {:?}", path))?;

        Ok(Self {
            created_at: FileTime::from_last_modification_time(&metadata).unix_seconds(),
            relative_path: relative_path.to_string(),
        })
    }

    pub fn created_at(&self) -> i64 {
        self.created_at
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}

impl TrackMetadata {
    pub fn load<P>(path: P, relative_path: &str) -> Result<Self>
    where
        P: Debug + AsRef<Path>,
    {
        let tag = get_tag(&path)
            .with_context(|| format!("failed to read id3 tags of path {:?}", path))?;
        let duration_seconds = duration_seconds(&tag, path);

        Ok(Self {
            tag,
            duration_seconds,
            relative_path: relative_path.to_string(),
        })
    }

    pub fn duration_seconds(&self) -> i64 {
        self.duration_seconds
    }

    pub fn title(&self) -> &str {
        self.tag.title().unwrap_or("?")
    }

    pub fn album(&self) -> &str {
        self.tag.album().unwrap_or("?")
    }

    pub fn artist(&self) -> &str {
        self.tag
            .artist()
            .or_else(|| self.tag.album_artist())
            .unwrap_or("?")
    }

    pub fn date(&self) -> String {
        self.tag
            .year()
            .or_else(|| self.tag.date_released().map(|d| d.year))
            .or_else(|| self.tag.date_recorded().map(|d| d.year))
            .map(|year| year.to_string())
            .unwrap_or_else(|| "?".to_string())
    }

    pub fn track_number(&self) -> String {
        let track = self
            .tag
            .track()
            .map(|num| num.to_string())
            .unwrap_or_else(|| "?".to_string());

        if let Some(disc) = self.tag.disc() {
            format!("{}.{}", disc, track)
        } else {
            track
        }
    }

    pub fn relative_path(&self) -> &str {
        &self.relative_path
    }
}

fn get_tag<P>(path: P) -> Result<Tag>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(path)?;
    if Tag::is_candidate(&mut file)? {
        Ok(Tag::read_from(file)?)
    } else if id3::v1::Tag::is_candidate(&mut file)? {
        Ok(id3::v1::Tag::read_from(file)?.into())
    } else {
        Err(anyhow!("file doesn't contain id3v1 or id3v2 tags"))
    }
}

fn duration_seconds<P>(tag: &Tag, path: P) -> i64
where
    P: AsRef<Path>,
{
    tag.duration()
        .map(|d| (d + 999) / 1000) // Convert ms to s, rounding up
        .or_else(|| duration_from_path(&path))
        .unwrap_or(0)
        .into()
}

fn duration_from_path<P>(path: P) -> Option<u32>
where
    P: AsRef<Path>,
{
    mp3_duration::from_path(&path)
        .map(|d| d.as_secs() as u32)
        .ok()
}
