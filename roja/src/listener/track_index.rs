use crate::library::TrackIndex;
use crate::ui::{Event, IntoListener, Listener};
use crate::util::channel;

pub struct TrackIndexListener {
    index: TrackIndex,
    sender: channel::Sender<Event>,
}

impl Listener for TrackIndexListener {
    fn on_event(&mut self, event: &Event) {
        if let Event::SelectFolder(folder_id) = event {
            self.sender.send(Event::LoadPlaylistFolder(folder_id));
        }
    }
}

impl IntoListener for TrackIndex {
    type LType = TrackIndexListener;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            index: self,
            sender,
        }
    }
}
