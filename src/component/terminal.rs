use std::convert::TryInto;

use crate::ui::{Event, Listener, State};
use crate::util::terminal;

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::init();
        Terminal
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::deinit();
    }
}

impl Listener for Terminal {
    fn on_event(&self, event: &Event, state: &mut State) {
        if *event == Event::Quit {
            state.stop();
        }
    }

    fn wait_event(&self) -> Option<Event> {
        if let Some(event) = terminal::poll_event() {
            event.try_into().ok()
        } else {
            None
        }
    }
}
