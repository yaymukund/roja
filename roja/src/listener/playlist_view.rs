use std::borrow::Cow;

use crate::listener::{ColumnWidth, List, ListBuilder, ListRow};
use crate::store::{get_tracks_by_folder_id, Track};
use crate::ui::{layout, Event, IntoListener, Section};
use crate::util::{channel, format_duration};

pub enum TrackColumn {
    TrackNumber,
    Title,
    Date,
    Duration,
}

impl ListRow for Track {
    type Column = TrackColumn;
    fn column_text(&self, column: &Self::Column) -> Cow<'_, str> {
        match column {
            TrackColumn::TrackNumber => Cow::Borrowed(&self.track_number),
            TrackColumn::Title => Cow::Borrowed(&self.title),
            TrackColumn::Date => Cow::Borrowed(&self.date),
            TrackColumn::Duration => Cow::Owned(format_duration(self.duration as i64)),
        }
    }
}

pub struct PlaylistView;

impl IntoListener for PlaylistView {
    type LType = List<Track>;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        ListBuilder::new(Vec::new())
            .section(Section::Playlist)
            .make_canvas(layout::playlist_canvas)
            .column(TrackColumn::TrackNumber, "#", ColumnWidth::Absolute(4))
            .column(TrackColumn::Title, "Title", ColumnWidth::Auto)
            .column(TrackColumn::Date, "Year", ColumnWidth::Absolute(4))
            .column(TrackColumn::Duration, "Length", ColumnWidth::Absolute(5))
            .on_select(move |index: usize, tracks: &[Track]| {
                let tracks = tracks[index..].iter().map(|t| t.id).collect();
                sender.send(Event::QueueTracks(tracks))
            })
            .on_event(|event: &Event, list: &mut List<Track>| {
                if let Event::LoadPlaylistFolder(folder_id) = event {
                    let tracks = get_tracks_by_folder_id(*folder_id)
                        .expect("could not find tracks for folder");
                    list.set_items(tracks);
                }
            })
            .build()
    }
}
