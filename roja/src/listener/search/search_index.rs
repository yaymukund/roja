use std::fs::File;
use std::sync::Arc;
use std::thread;

use anyhow::Result;
use fst::automaton::Subsequence;
use fst::IntoStreamer;
use fst::Map;
use memmap::Mmap;

use crate::util::channel;
use crate::SETTINGS;

pub enum SearchEvent {
    Quit,
    Search(Arc<String>),
}

pub type SearchResult = Vec<u64>;

pub struct SearchIndex {
    fst: Map<Mmap>,
    sender: channel::Sender<SearchResult>,
    receiver: channel::Receiver<SearchEvent>,
}

impl SearchIndex {
    fn new(
        sender: channel::Sender<SearchResult>,
        receiver: channel::Receiver<SearchEvent>,
    ) -> Result<Self> {
        let search_index_path = SETTINGS.with(|s| s.place_search_index_file());
        let f = File::open(search_index_path)?;
        let mmap = unsafe { Mmap::map(&f)? };
        let fst = Map::new(mmap)?;
        Ok(Self {
            fst,
            sender,
            receiver,
        })
    }

    fn search(&self, text: &str) {
        // If there are more recent events, we can abandon processing the current event.
        if !self.receiver.is_empty() {
            return;
        }

        let input = Subsequence::new(text);
        let ids = self.fst.search(input).into_stream().into_values();
        self.display_results(ids);
    }

    fn display_results(&self, ids: Vec<u64>) {
        // If there are more recent events, we can abandon processing the current event.
        if !self.receiver.is_empty() {
            return;
        }

        self.sender
            .send(ids)
            .expect("could not send event to disconnected channel");
    }

    fn run(&self) {
        loop {
            match self.receiver.recv() {
                Ok(SearchEvent::Quit) => break,
                Ok(SearchEvent::Search(term)) => self.search(&term),
                Err(_) => {
                    // TODO disconnect properly before quitting
                }
            }
        }
    }
}

pub fn spawn_searcher() -> Result<(
    channel::Sender<SearchEvent>,
    channel::Receiver<SearchResult>,
)> {
    let (tx_events, rx_events) = channel::unbounded();
    let (tx_results, rx_results) = channel::unbounded();
    let search_index = SearchIndex::new(tx_results, rx_events)?;

    thread::spawn(move || {
        search_index.run();
    });

    Ok((tx_events, rx_results))
}
