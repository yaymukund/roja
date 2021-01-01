use std::fs::File;

use anyhow::Result;
use fst::Map;
use memmap::Mmap;

use crate::store::Playlist;
use crate::ui::{Event, IntoListener, Listener};
use crate::util::channel;
use crate::SETTINGS;

pub struct Search;

// holds the search index, responds to Search events by sending QueuePlaylist events
pub struct SearchListener {
    sender: channel::Sender<Event>,
    fst: Map<Mmap>,
}

impl SearchListener {
    fn new(sender: channel::Sender<Event>) -> Result<Self> {
        let search_index_path = SETTINGS.with(|s| s.place_search_index_file());
        let f = File::open(search_index_path)?;
        let mmap = unsafe { Mmap::map(&f)? };
        let fst = Map::new(mmap)?;
        Ok(Self { sender, fst })
    }
}

impl Listener for SearchListener {
    fn on_event(&mut self, event: &Event) {
        match event {
            _ => {}
        }
    }
}

impl IntoListener for Search {
    type LType = SearchListener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType::new(sender.clone()).expect("could not initialize search index")
    }
}
