use std::rc::Rc;

use super::{List, ListRow};
use crate::library::{Playlist, Track};
use crate::ui::{Event, IntoListener, Layout, Listener, State};

impl ListRow for Rc<Track> {
    fn row_text(&self) -> &str {
        self.title()
    }
}

pub struct PlaylistView {
    list: List,
    playlist: Playlist,
}

impl Listener for PlaylistView {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        let mut event = event;
        if let Event::SetPlaylistTracks(tracks) = event {
            let playlist_tracks = self.playlist.tracks();
            playlist_tracks.drain(..);
            playlist_tracks.extend_from_slice(tracks);
            self.list.reset();

            event = &Event::Draw;
        }

        self.list
            .items(&self.playlist.tracks())
            .process_event(event, ui);
    }
}

impl IntoListener for Playlist {
    type LType = PlaylistView;

    fn into_listener(self, layout: &Layout) -> Self::LType {
        let mut list = List::new(layout, |layout| layout.playlist.clone());
        list.disable();

        Self::LType {
            list,
            playlist: self,
        }
    }
}
