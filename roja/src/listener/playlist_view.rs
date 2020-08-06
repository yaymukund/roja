use std::borrow::Cow;
use std::rc::Rc;

use crate::listener::{ColumnWidth, List, ListBuilder, ListRow};
use crate::store::{Playlist, Track};
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
    type LType = List<Track, Playlist>;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        ListBuilder::new(Playlist {
            tracks: Rc::new(Vec::new()),
            selected_index: 0,
        })
        .section(Section::Playlist)
        .make_canvas(layout::playlist_canvas)
        .column(TrackColumn::TrackNumber, "#", ColumnWidth::Absolute(4))
        .column(TrackColumn::Title, "Title", ColumnWidth::Auto)
        .column(TrackColumn::Date, "Year", ColumnWidth::Absolute(4))
        .column(TrackColumn::Duration, " ◴", ColumnWidth::Absolute(5))
        .on_highlight(|index: usize, playlist: &mut Playlist| {
            playlist.selected_index = index;
        })
        .on_select(move |_index: usize, playlist: &mut Playlist| {
            let event = Event::QueuePlaylist(playlist.clone());
            sender.send(event);
        })
        .on_event(|event: &Event, list: &mut List<Track, Playlist>| {
            if let Event::DisplayPlaylist(playlist) = event {
                list.set_items(playlist.clone());
            }
        })
        .build()
    }
}
