use gumdrop::Options;

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

#[derive(Debug, Options)]
pub struct InitDbOpts {}
