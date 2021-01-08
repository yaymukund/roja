mod search_index;

use anyhow::Result;
use std::sync::Arc;

// use crate::store::Playlist;
use crate::ui::{Event, IntoListener, Listener};
use crate::util::channel;
use search_index::{spawn_searcher, SearchEvent, SearchResult};

pub struct Search;

// holds the search index, responds to Search events by sending QueuePlaylist events
pub struct SearchListener {
    sender: channel::Sender<Event>,
    index_sender: channel::Sender<SearchEvent>,
    index_receiver: channel::Receiver<SearchResult>,
}

impl SearchListener {
    fn new(sender: channel::Sender<Event>) -> Result<Self> {
        let (index_sender, index_receiver) = spawn_searcher()?;
        Ok(Self {
            sender,
            index_sender,
            index_receiver,
        })
    }

    fn tick(&mut self) {
        match self.index_receiver.try_recv() {
            Ok(results) => println!("{:?}", results),
            Err(channel::TryRecvError::Disconnected) => panic!("disconnected before quitting"),
            Err(channel::TryRecvError::Empty) => {}
        }
    }
}

impl Listener for SearchListener {
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Tick => self.tick(),
            Event::Quit => self.index_sender.send(SearchEvent::Quit)?,
            Event::Search(term) if term.len() > 2 => {
                let event = SearchEvent::Search(Arc::new(term.to_string()));
                self.index_sender.send(event)?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl IntoListener for Search {
    type LType = SearchListener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType::new(sender.clone()).expect("could not initialize search index")
    }
}
