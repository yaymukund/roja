mod search_index;

use anyhow::Result;
use std::rc::Rc;
use std::sync::Arc;

use crate::store::{get_tracks_by_id, Playlist};
use crate::ui::{Event, IntoListener, Listener};
use crate::util::{channel, SendDiscard};
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

    fn display_results(&self, ids: Vec<u64>) -> Result<()> {
        let tracks = if ids.is_empty() {
            vec![]
        } else {
            get_tracks_by_id(&ids)?
        };

        self.sender.send_discard(Event::DisplayPlaylist(Playlist {
            tracks: Rc::new(tracks),
            selected_index: 0,
        }))
    }

    fn tick(&mut self) -> Result<()> {
        match self.index_receiver.try_recv() {
            Ok(ids) => self.display_results(ids)?,
            Err(channel::TryRecvError::Disconnected) => panic!("disconnected before quitting"),
            Err(channel::TryRecvError::Empty) => {}
        }

        Ok(())
    }
}

impl Listener for SearchListener {
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::Tick => self.tick()?,
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
