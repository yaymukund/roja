use std::path::PathBuf;
use structopt::StructOpt;

// A command line music player written in Rust.
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct Cli {
    // Path to the configuration file.
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    pub config_path: Option<PathBuf>,
}

impl Cli {
    pub fn from_args() -> Self {
        <Cli as StructOpt>::from_args()
    }
}
