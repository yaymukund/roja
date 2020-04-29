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

impl Folder {
    pub fn path_str(&self) -> &str {
        self.path.to_str().expect("could not turn path into string")
    }
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
pub struct Store {
    pub tracks: Vec<Track>,
    pub folders: Vec<Folder>,
}

impl Store {
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = File::open(path).expect("Could not open metadata file");
        let reader = BufReader::new(file);
        let store: Store =
            serde_json::from_reader(reader).expect("Could not read library JSON file");

        println!(
            "Loaded {} tracks and {} folders",
            store.tracks.len(),
            store.folders.len()
        );

        store
    }
}
