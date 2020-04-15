use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct Folder {
    id: usize,
    pub(crate) created_at: usize,
    pub(crate) path: PathBuf,
}

impl Folder {
    pub(crate) fn path_str(&self) -> &str {
        self.path.to_str().expect("could not turn path into string")
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Track {
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
pub(crate) struct Library {
    tracks: Vec<Track>,
    pub(crate) folders: Vec<Folder>,
}

impl Library {
    pub(crate) fn from_path<P>(path: P) -> Library
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
