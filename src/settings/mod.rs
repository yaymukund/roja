mod cli;
mod colors;
mod config;
mod deserialize_color;

use once_cell::sync::OnceCell;

use cli::Cli;
pub use colors::Colors;
use config::Config;
pub use deserialize_color::SColor;
use std::path::PathBuf;

static SETTINGS: OnceCell<Settings> = OnceCell::new();

#[derive(Debug)]
pub struct Settings {
    cli: Cli,
    config: Config,
    xdg: xdg::BaseDirectories,
}

impl Settings {
    const CONFIG_PATH: &'static str = "config.json";

    pub fn global() -> &'static Self {
        SETTINGS.get().expect("settings used before initialization")
    }

    pub fn init() {
        let settings = Self::new();
        SETTINGS.set(settings).unwrap();
    }

    fn new() -> Self {
        let xdg =
            xdg::BaseDirectories::with_prefix("roja").expect("Could not initialize directories");

        let cli = Cli::from_args();

        let config_path = cli
            .config_path
            .unwrap_or_else(|| xdg.place_config_file(Self::CONFIG_PATH).unwrap());

        Settings {
            cli: Cli::from_args(),
            config: Config::from_path(config_path),
            xdg,
        }
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
