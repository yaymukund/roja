use crate::ui::{Event, IntoListener, KeyCode, Listener};
use crate::util::{channel, SendDiscard};
use anyhow::Result;

pub struct Focus;

pub struct FocusListener {
    opened_playlist: bool,
    searching: bool,
    sender: channel::Sender<Event>,
}

impl FocusListener {
    fn focus_next_tab(&mut self) -> Result<()> {
        let next = if self.opened_playlist {
            Event::FocusFolderList
        } else {
            Event::FocusPlaylist
        };

        self.sender.send_discard(next)?;
        self.opened_playlist = !self.opened_playlist;
        Ok(())
    }

    fn open_search(&mut self) -> Result<()> {
        self.searching = true;
        self.opened_playlist = false;
        self.sender.send_discard(Event::FocusSearch)
    }

    fn cancel_search(&mut self) -> Result<()> {
        self.searching = false;
        self.sender.send_discard(Event::CancelSearch)
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
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match (self.searching, event.keycode()) {
            (true, Some(KeyCode::Esc)) => self.cancel_search()?,
            (true, _) => {}
            (false, Some(KeyCode::Char('/'))) => self.open_search()?,
            (false, Some(KeyCode::Char('q'))) => self.sender.send_discard(Event::Quit)?,
            (false, Some(KeyCode::Tab)) => self.focus_next_tab()?,
            _ => {}
        }

        Ok(())
    }
}
