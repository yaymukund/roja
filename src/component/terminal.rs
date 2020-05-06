use std::convert::TryInto;

use crate::ui::{Event, Listener, State};
use crate::util::terminal;

pub struct Terminal;

impl Terminal {
    pub fn new() -> Self {
        terminal::init();
        Terminal
    }

    fn next_event(&self) -> Option<Event> {
        terminal::poll_event().and_then(|ev| ev.try_into().ok())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::deinit();
    }
}

impl Listener for Terminal {
    fn on_event(&mut self, event: &Event, state: &mut State) {
        match *event {
            Event::Tick => {
                if let Some(event) = self.next_event() {
                    state.dispatch(event);
                }
            }
            Event::Resize(cols, rows) => {
                terminal::clear_all();
                state.resize(cols, rows);
            }
            Event::Quit => state.stop(),
            _ => {}
        }
    }
}
