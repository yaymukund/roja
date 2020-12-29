use crate::ui::{Event, IntoListener, KeyCode, Listener};
use crate::util::channel;

pub struct Focus;

pub struct FocusListener {
    opened_playlist: bool,
    searching: bool,
    sender: channel::Sender<Event>,
}

impl FocusListener {
    fn focus_next_tab(&mut self) {
        let next = if self.opened_playlist {
            Event::OpenFolderList
        } else {
            Event::OpenPlaylist
        };

        self.sender.send(next);
        self.opened_playlist = !self.opened_playlist;
    }

    fn open_search(&mut self) {
        self.searching = true;
        self.opened_playlist = false;
        self.sender.send(Event::OpenSearch);
    }

    fn close_search(&mut self) {
        self.searching = false;
        self.sender.send(Event::CloseSearch);
    }
}

impl IntoListener for Focus {
    type LType = FocusListener;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            opened_playlist: false,
            searching: false,
            sender,
        }
    }
}

impl Listener for FocusListener {
    fn on_event(&mut self, event: &Event) {
        match (self.searching, event.keycode()) {
            (true, Some(KeyCode::Esc)) => self.close_search(),
            (true, _) => return,
            (false, Some(KeyCode::Char('/'))) => self.open_search(),
            (false, Some(KeyCode::Char('q'))) => self.sender.send(Event::Quit),
            (false, Some(KeyCode::Tab)) => self.focus_next_tab(),
            _ => {}
        }
    }
}
