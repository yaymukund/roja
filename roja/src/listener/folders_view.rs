use std::borrow::Cow;

use super::{ColumnWidth, List, ListBuilder, ListRow};
use crate::store::{get_folders, Folder};
use crate::ui::{layout, Event, IntoListener, Section};
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
    type LType = List<Folder>;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let folders = get_folders().expect("could not get folders from db");

        ListBuilder::new(folders)
            .section(Section::FoldersList)
            .autofocus()
            .column(FolderColumn, "Folders", ColumnWidth::Auto)
            .make_canvas(layout::folders_view_canvas)
            .on_highlight(move |index: usize, folders: &[Folder]| {
                let folder_id = folders[index].id;
                sender.send(Event::LoadPlaylistFolder(folder_id));
            })
            .build()
    }
}
