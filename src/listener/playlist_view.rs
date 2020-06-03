use std::rc::Rc;

use crate::library::{Playlist, Track};
use crate::listener::{List, ListBuilder, ListRenderer, ListRow};
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

impl ListRow for Rc<Track> {
    fn row_text(&self) -> &str {
        self.title()
    }
}

pub struct PlaylistView {
    list: List<Rc<Track>>,
    playlist: Playlist,
}

impl PlaylistView {
    fn set_playlist_tracks(&mut self, tracks: &[Rc<Track>]) {
        let playlist_tracks = self.playlist.tracks();
        playlist_tracks.drain(..);
        playlist_tracks.extend_from_slice(tracks);
        self.list.reset();
    }

    fn with_renderer<F>(&mut self, f: F)
    where
        F: Fn(&mut ListRenderer<'_, Rc<Track>>),
    {
        let tracks = self.playlist.tracks();
        self.list.with_items(&tracks, |renderer| f(renderer));
    }
}

impl Listener for PlaylistView {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::SetPlaylistTracks(tracks) => {
                self.set_playlist_tracks(tracks);
                self.with_renderer(|e| e.draw());
            }
            _ => self.with_renderer(|e| e.process_event(event)),
        }
    }
}

impl IntoListener for Playlist {
    type LType = PlaylistView;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let list = ListBuilder::new()
            .section(Section::Playlist)
            .make_canvas(layout::playlist_canvas)
            .on_select(move |index: usize, tracks: &[Rc<Track>]| {
                let tracks = tracks[index..].to_vec();
                sender.send(Event::QueueTracks(tracks))
            })
            .build();

        Self::LType {
            list,
            playlist: self,
        }
    }
}
