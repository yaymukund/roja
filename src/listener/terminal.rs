use std::convert::TryInto;

use crate::ui::{Event, Listener, State};
use crate::util::terminal;

pub struct Terminal;

impl Listener for Terminal {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        match *event {
            Event::Tick => self.wait_event(ui),
            Event::Quit => ui.stop(),
            Event::Resize(cols, rows) => {
                terminal::clear_all();
                ui.resize(cols, rows);
            }
            _ => {}
        }
    }
}

impl Terminal {
    pub fn new() -> Self {
        terminal::init();
        Terminal
    }

    fn next_event(&self) -> Option<Event> {
        terminal::poll_event().and_then(|ev| ev.try_into().ok())
    }

    fn wait_event(&self, ui: &mut State) {
        if let Some(event) = self.next_event() {
            ui.dispatch(event);
        }
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::deinit();
    }
}
