use std::convert::TryInto;

use crate::ui::{Component, Event, State};
use crate::util::terminal;

pub struct Terminal;

impl Component for Terminal {
    fn on_tick(&self, ui: &mut State) {
        if let Some(event) = self.next_event() {
            ui.dispatch(event);
        }
    }

    fn on_event(&mut self, event: &Event, ui: &mut State) {
        match *event {
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
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::deinit();
    }
}
