use std::rc::Rc;

use crate::library::{Playlist, Track};
use crate::listener::{ColumnWidth, List, ListBuilder, ListRenderer, ListRow};
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

pub enum TrackColumn {
    TrackNumber,
    Title,
    Date,
    Duration,
}

impl ListRow for Rc<Track> {
    type Column = TrackColumn;
    fn column_text(&self, column: &Self::Column) -> &str {
        match column {
            TrackColumn::TrackNumber => self.track_number(),
            TrackColumn::Title => self.title(),
            TrackColumn::Date => self.date(),
            TrackColumn::Duration => self.duration(),
        }
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
            .column(TrackColumn::TrackNumber, "#", ColumnWidth::Absolute(4))
            .column(TrackColumn::Title, "Title", ColumnWidth::Auto)
            .column(TrackColumn::Date, "Year", ColumnWidth::Absolute(4))
            .column(TrackColumn::Duration, "Length", ColumnWidth::Absolute(4))
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
