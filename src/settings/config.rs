use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_library_path: PathBuf,
    pub metadata_path: PathBuf,
    pub metadata_index_path: PathBuf,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(&path).unwrap_or_else(move |_| {
            panic!(format!("Could not open {}", path.as_ref().display()));
        });
        let reader = BufReader::new(file);
        // We want it to panic if the json is invalid.
        serde_json::from_reader(reader).unwrap()
    }
}
