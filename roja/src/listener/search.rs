use crate::store::Playlist;
use crate::ui::{Event, IntoListener, Listener, Section};
use crate::util::channel;

pub struct Search;

// holds the search index, responds to Search events by sending QueuePlaylist events
pub struct SearchListener {
    sender: channel::Sender<Event>,
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
        Self::LType {
            sender: sender.clone(),
        }
    }
}
