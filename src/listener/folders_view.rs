use super::{List, ListRow};
use crate::library::Folder;
use crate::ui::{Event, IntoListener, Layout, Listener, State};

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

pub struct FoldersView {
    list: List,
    folders: Vec<Folder>,
}

impl Listener for FoldersView {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        self.list
            .items(&self.folders)
            .on_highlight(|folder, ui| {
                ui.dispatch(Event::SelectFolder(folder.id()));
            })
            .process_event(event, ui);
    }
}

impl IntoListener for Vec<Folder> {
    type LType = FoldersView;

    fn into_listener(self, layout: &Layout) -> Self::LType {
        let list = List::new(layout, |layout| layout.folder_view.clone());

        Self::LType {
            list,
            folders: self,
        }
    }
}
