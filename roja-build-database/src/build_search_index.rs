use anyhow::Result;
use fst::MapBuilder;

use std::fs::File;
use std::io;
use std::path::PathBuf;

use crate::connection::Connection;

pub struct BuildSearchIndex<'a> {
    pub conn: &'a Connection,
    pub outfile: PathBuf,
}

impl<'a> BuildSearchIndex<'a> {
    pub fn execute(&self) -> Result<()> {
        let file = File::create(&self.outfile)?;
        let wtr = io::BufWriter::new(file);
        let mut builder = MapBuilder::new(wtr)?;

        let search_strings = self.conn.select_all_track_search_strings()?;
        for (id, search_string) in search_strings {
            builder.insert(search_string, id as u64)?;
        }

        builder.finish()?;
        Ok(())
    }
}
