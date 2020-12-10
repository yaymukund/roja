use crate::ui::{Event, IntoListener, Listener, Section};
use crate::util::channel;

pub struct Focus;

pub struct FocusListener {
    focused_section: Section,
    sender: channel::Sender<Event>,
}

impl FocusListener {
    fn focus_next_tab(&mut self) {
        let next = if self.focused_section == Section::FoldersList {
            Section::Playlist
        } else {
            Section::FoldersList
        };

        self.sender.send(Event::Focus(next));
        self.focused_section = next;
    }

    fn focus_search(&mut self) {
        self.sender.send(Event::Focus(Section::Search));
    }
}

impl IntoListener for Focus {
    type LType = FocusListener;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            focused_section: Section::FoldersList,
            sender,
        }
    }
}

impl Listener for FocusListener {
    fn on_event(&mut self, event: &Event) {
        if event.is_char_press('/') {
            self.focus_search();
        } else if event.is_tab() {
            self.focus_next_tab();
        }
    }
}
