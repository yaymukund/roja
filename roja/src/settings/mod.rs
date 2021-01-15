mod colors;
mod config;
mod deserialize_color;

use std::path::PathBuf;

use anyhow::Result;
use xdg::BaseDirectories;

pub use colors::Colors;
use config::Config;
pub use deserialize_color::SColor;

pub const SEARCH_RESULTS_LIMIT: usize = 200;

static BASEDIRECTORY_PREFIX: &str = "roja";
static FILENAME_CONFIG: &str = "config.toml";
static FILENAME_DB: &str = "roja-store.db";
static FILENAME_SEARCH_INDEX: &str = "roja-search-index";

thread_local! {
    pub static SETTINGS: Settings = Settings::build().unwrap();
}

#[derive(Debug)]
pub struct Settings {
    config: Config,
    xdg: BaseDirectories,
}

impl Settings {
    pub fn build() -> Result<Self> {
        let xdg = BaseDirectories::with_prefix(BASEDIRECTORY_PREFIX)?;
        let config_path = xdg.place_config_file(FILENAME_CONFIG)?;

        Ok(Settings {
            xdg,
            config: Config::from_path(config_path)?,
        })
    }

    pub fn place_db_file(&self) -> PathBuf {
        self.xdg
            .place_data_file(FILENAME_DB)
            .expect("could not place database file")
    }

    pub fn place_search_index_file(&self) -> PathBuf {
        self.xdg
            .place_data_file(FILENAME_SEARCH_INDEX)
            .expect("could not place search index file")
    }

    pub fn music_library_path(&self) -> &PathBuf {
        &self.config.music_library_path
    }

    pub fn colors(&self) -> &Colors {
        &self.config.colors
    }
}
