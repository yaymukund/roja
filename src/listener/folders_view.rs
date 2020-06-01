use super::{List, ListBuilder, ListRow};
use crate::library::Folder;
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

pub struct FoldersView {
    list: List<Folder>,
    folders: Vec<Folder>,
}

impl Listener for FoldersView {
    fn on_event(&mut self, event: &Event) {
        self.list
            .with_items(&self.folders, |r| r.process_event(event));
    }
}

impl IntoListener for Vec<Folder> {
    type LType = FoldersView;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let list = ListBuilder::new()
            .section(Section::FoldersList)
            .autofocus()
            .make_canvas(layout::folders_view_canvas)
            .on_highlight(move |folder: &Folder| {
                sender.send(Event::SelectFolder(folder.id()));
            })
            .build();

        Self::LType {
            list,
            folders: self,
        }
    }
}
