use std::path::PathBuf;
use structopt::StructOpt;

// A command line music player written in Rust.
#[derive(StructOpt, Debug)]
#[structopt(name = "lucrecia")]
pub struct Config {
    // Path to your music library.
    #[structopt(short = "l", long = "music-library", parse(from_os_str))]
    music_library_path: PathBuf,
}

impl Config {
    pub fn new() -> Config {
        Config::from_args()
    }
}
