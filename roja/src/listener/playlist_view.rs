use crate::library::Playlist;
use crate::listener::{ColumnWidth, List, ListBuilder, ListRenderer, ListRow};
use crate::store::{get_tracks_by_folder_id, Track};
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

pub enum TrackColumn {
    TrackNumber,
    Title,
    Date,
    Duration,
}

impl ListRow for Track {
    type Column = TrackColumn;
    fn column_text(&self, column: &Self::Column) -> &str {
        match column {
            TrackColumn::TrackNumber => &self.track_number,
            TrackColumn::Title => &self.title,
            TrackColumn::Date => &self.date,
            TrackColumn::Duration => &self.duration,
        }
    }
}

pub struct PlaylistView {
    list: List<Track>,
    playlist: Playlist,
}

impl PlaylistView {
    fn set_playlist_tracks(&mut self, tracks: Vec<Track>) {
        let playlist_tracks = self.playlist.tracks();
        playlist_tracks.clear();
        playlist_tracks.extend(tracks);
        self.list.reset();
    }

    fn with_renderer<F>(&mut self, f: F)
    where
        F: Fn(&mut ListRenderer<'_, Track>),
    {
        let tracks = self.playlist.tracks();
        self.list.with_items(&tracks, |renderer| f(renderer));
    }
}

impl Listener for PlaylistView {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::LoadPlaylistFolder(folder_id) => {
                let tracks =
                    get_tracks_by_folder_id(*folder_id).expect("could not find tracks for folder");
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
            .on_select(move |index: usize, tracks: &[Track]| {
                let tracks = tracks[index..].iter().map(|t| t.id).collect();
                sender.send(Event::QueueTracks(tracks))
            })
            .build();

        Self::LType {
            list,
            playlist: self,
        }
    }
}
