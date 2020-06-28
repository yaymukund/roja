use gumdrop::Options;

use std::path::PathBuf;

/// A command line music player written in Rust.
#[derive(Debug, Options)]
pub struct CliOptions {
    #[options()]
    pub help: bool,

    #[options(command)]
    pub command: Option<Command>,
}

#[derive(Debug, Options)]
pub enum Command {
    /// initialize the music database and exit
    #[options()]
    InitDb(InitDbOpts),
}

/// initialize the music database and exit
#[derive(Debug, Options)]
pub struct InitDbOpts {
    #[options(required, short = "l")]
    pub load_path: PathBuf,

    #[options()]
    pub create: bool,

    #[options()]
    pub help: bool,
}
