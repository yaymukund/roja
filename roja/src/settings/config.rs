use anyhow::{anyhow, Result};
use serde::Deserialize;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::Colors;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub music_library_path: PathBuf,
    pub colors: Colors,
}

impl Config {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut config_str = String::new();
        file.read_to_string(&mut config_str)?;
        toml::from_str(&config_str).map_err(|_| anyhow!("could not parse toml from config.toml"))
    }
}
