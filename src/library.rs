use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Folder {
    id: usize,
    pub created_at: usize,
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    id: usize,
    title: String,
    album: String,
    artist: String,
    date: String,
    track_number: String,
    duration: String,
    path: PathBuf,
    folder_id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    tracks: Vec<Track>,
    pub folders: Vec<Folder>,
}

impl Library {
    pub fn from_path<P>(path: P) -> Library
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).expect("Could not open metadata file");
        let reader = BufReader::new(file);
        let library: Library =
            serde_json::from_reader(reader).expect("Could not read library JSON file");

        println!(
            "Loaded {} tracks and {} folders",
            library.tracks.len(),
            library.folders.len()
        );

        library
    }
}
