use super::{List, ListRow};
use crate::library::Folder;
use crate::ui::{layout, Event, IntoListener, Listener, Section};
use crate::util::channel;

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

pub struct FoldersView {
    list: List,
    folders: Vec<Folder>,
    sender: channel::Sender<Event>,
}

impl Listener for FoldersView {
    fn on_event(&mut self, event: &Event) {
        let sender = self.sender.clone();

        self.list
            .items(&self.folders)
            .on_highlight(move |folder| {
                sender.send(Event::SelectFolder(folder.id()));
            })
            .process_event(event);
    }
}

impl IntoListener for Vec<Folder> {
    type LType = FoldersView;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        let list = List::new(Section::FoldersList, layout::folders_view_canvas);

        Self::LType {
            list,
            folders: self,
            sender,
        }
    }
}
