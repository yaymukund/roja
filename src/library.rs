mod folder;
mod track;

use folder::Folder;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use track::Track;

#[derive(Serialize, Deserialize, Debug)]
pub struct Library {
    tracks: Vec<Track>,
    folders: Vec<Folder>,
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
