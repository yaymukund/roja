use super::{List, ListRow};
use crate::library::Folder;
use crate::ui::{Event, IntoListener, Listener, State};
use crate::util::Canvas;

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

pub struct FolderList {
    list: List,
    folders: Vec<Folder>,
}

impl Listener for FolderList {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        self.list
            .with(&self.folders)
            .on_highlight(|folder, ui| {
                ui.dispatch(Event::SelectFolder(folder.id()));
            })
            .process_event(event, ui);
    }
}

impl IntoListener for Vec<Folder> {
    type LType = FolderList;

    fn into_listener(self, cols: u16, rows: u16) -> Self::LType {
        let list = List::new(cols, rows, |cols, rows| {
            Canvas::new(point!(0, 0), cols / 3, rows - 1)
        });

        Self::LType {
            list,
            folders: self,
        }
    }
}
