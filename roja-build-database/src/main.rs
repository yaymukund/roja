#![warn(rust_2018_idioms)]
mod build_database;
mod build_search_index;
mod connection;
mod metadata;
mod progress;

use anyhow::Result;
use gumdrop::Options;

use std::path::PathBuf;

use build_database::BuildDatabase;
use build_search_index::BuildSearchIndex;
use connection::Connection;

/// build the roja music database
#[derive(Options)]
struct CliOpts {
    /// directory containing mp3s
    #[options(required, short = "d")]
    pub root_directory: PathBuf,

    /// output filename
    #[options(required, short = "o")]
    pub out_file: PathBuf,

    /// create tables
    #[options(short = "c")]
    pub create: bool,

    /// if specified, save search index to path
    #[options(short = "s")]
    pub search_db_file: Option<PathBuf>,

    #[options()]
    pub help: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = CliOpts::parse_args_default_or_exit();
    let conn = Connection::open(cli.out_file)?;

    BuildDatabase {
        conn: &conn,
        root: cli.root_directory,
        create: cli.create,
    }
    .execute()?;

    if let Some(path) = cli.search_db_file {
        println!("Building search index...");
        BuildSearchIndex {
            conn: &conn,
            outfile: path,
        }
        .execute()?;
    }

    Ok(())
}
