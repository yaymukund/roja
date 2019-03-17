use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    music_library_path: PathBuf,
    metadata_path: PathBuf,
    metadata_index_path: PathBuf,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Config {
        let file = File::open(&path).unwrap_or_else(move |_| {
            panic!(format!("Could not open {}", path.as_ref().display()));
        });
        let reader = BufReader::new(file);
        // We want it to panic if the json is invalid.
        let config = serde_json::from_reader(reader).unwrap();
        dbg!(&config);
        config
    }
}
