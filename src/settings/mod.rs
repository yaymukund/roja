mod cli_options;
mod colors;
mod config;
mod deserialize_color;

use std::path::PathBuf;

use xdg::BaseDirectories;

pub use cli_options::{CliOptions, Command};
pub use colors::Colors;
use config::Config;
pub use deserialize_color::SColor;

static BASEDIRECTORY_PREFIX: &str = "roja";
static FILENAME_CONFIG: &str = "config.json";
static FILENAME_DB: &str = "store.db";

thread_local! {
    pub static SETTINGS: Settings = Settings::new();
}

#[derive(Debug)]
pub struct Settings {
    config: Config,
    xdg: BaseDirectories,
}

impl Settings {
    pub fn new() -> Self {
        let xdg = BaseDirectories::with_prefix(BASEDIRECTORY_PREFIX)
            .expect("could not initialize xdg dir");

        let config_path = xdg
            .place_config_file(FILENAME_CONFIG)
            .expect("could not place config file");

        Settings {
            xdg,
            config: Config::from_path(config_path),
        }
    }

    pub fn place_db_file(&self) -> PathBuf {
        self.xdg
            .place_data_file(FILENAME_DB)
            .expect("could not place database file")
    }

    pub fn metadata_path(&self) -> &PathBuf {
        &self.config.metadata_path
    }

    pub fn music_library_path(&self) -> &PathBuf {
        &self.config.music_library_path
    }

    pub fn colors(&self) -> &Colors {
        &self.config.colors
    }
}
