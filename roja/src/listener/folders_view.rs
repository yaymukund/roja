use std::borrow::Cow;
use std::rc::Rc;

use super::{ColumnWidth, List, ListBuilder, ListRow};
use crate::store::{get_folders, get_tracks_by_folder_id, Folder, Playlist};
use crate::ui::{layout, Event, IntoListener};
use crate::util::channel;

pub struct FolderColumn;

impl ListRow for Folder {
    type Column = FolderColumn;
    fn column_text(&self, _column: &Self::Column) -> Cow<'_, str> {
        Cow::Borrowed(self.path_str())
    }
}

pub struct FoldersView;

impl IntoListener for FoldersView {
    type LType = List<Folder, Vec<Folder>>;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let folders = get_folders().expect("could not get folders from db");

        ListBuilder::new(folders)
            .autofocus()
            .column(FolderColumn, "Folders", ColumnWidth::Auto)
            .make_canvas(layout::folders_view_canvas)
            .on_highlight(move |index: usize, folders: &mut Vec<Folder>| {
                let folder_id = folders[index].id;
                let tracks =
                    get_tracks_by_folder_id(folder_id).expect("could not find tracks for folder");
                sender
                    .send(Event::DisplayPlaylist(Playlist {
                        tracks: Rc::new(tracks),
                        selected_index: 0,
                    }))
                    .expect("could not send event to disconnected channel");
            })
            .on_event(|event: &Event, list: &mut Self::LType| match event {
                Event::OpenPlaylist | Event::OpenSearch => list.unfocus(),
                Event::OpenFolderList | Event::CloseSearch => list.focus(),
                _ => {}
            })
            .build()
    }
}
