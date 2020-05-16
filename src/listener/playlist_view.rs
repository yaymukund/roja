use std::rc::Rc;

use super::{List, ListRow};
use crate::library::{Playlist, Track};
use crate::ui::{Event, IntoListener, Listener, State};
use crate::util::Canvas;

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

            event = &Event::Draw;
        }
        self.list
            .with(&self.playlist.tracks())
            .process_event(event, ui);
    }
}

impl IntoListener for Playlist {
    type LType = PlaylistView;

    fn into_listener(self, cols: u16, rows: u16) -> Self::LType {
        let list = List::new(cols, rows, |cols, rows| {
            let start_x = cols / 3 + 1;
            let width = cols - start_x;
            Canvas::new(point!(start_x, 0), width, rows - 1)
        });

        Self::LType {
            list,
            playlist: self,
        }
    }
}
