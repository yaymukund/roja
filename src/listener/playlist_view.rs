use std::rc::Rc;

use super::{List, ListRow};
use crate::library::{Playlist, Track};
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

impl ListRow for Rc<Track> {
    fn row_text(&self) -> &str {
        self.title()
    }
}

pub struct PlaylistView {
    sender: channel::Sender<Event>,
    list: List,
    playlist: Playlist,
}

impl Listener for PlaylistView {
    fn on_event(&mut self, event: &Event) {
        let mut event = event;
        if let Event::SetPlaylistTracks(tracks) = event {
            let playlist_tracks = self.playlist.tracks();
            playlist_tracks.drain(..);
            playlist_tracks.extend_from_slice(tracks);
            self.list.reset();

            event = &Event::Draw;
        }

        let sender = self.sender.clone();
        self.list
            .items(&self.playlist.tracks())
            .on_select(move |track: &Rc<Track>| {
                sender.send(Event::PlayTrack(track.clone()));
            })
            .process_event(event);
    }
}

impl IntoListener for Playlist {
    type LType = PlaylistView;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let mut list = List::new(Section::Playlist, layout::playlist_canvas);
        list.disable();

        Self::LType {
            sender,
            list,
            playlist: self,
        }
    }
}
