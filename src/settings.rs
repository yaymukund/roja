mod config;
use config::Config;
mod cli;
use cli::Cli;
use std::path::PathBuf;

pub(crate) struct Settings {
    cli: Cli,
    config: Config,
    xdg: xdg::BaseDirectories,
}

impl Settings {
    const CONFIG_PATH: &'static str = "config.json";

    pub(crate) fn new() -> Settings {
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

    pub(crate) fn metadata_path(&self) -> &PathBuf {
        &self.config.metadata_path
    }
}
