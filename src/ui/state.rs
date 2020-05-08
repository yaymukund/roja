use super::Event;
use crate::util::terminal;

pub struct State {
    events: Vec<Event>,
    stopped: bool,
    rows: u16,
    cols: u16,
}

impl State {
    pub fn new() -> Self {
        let (cols, rows) = terminal::size();
        State {
            events: Vec::new(),
            stopped: false,
            rows,
            cols,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.dispatch(Event::Resize(cols, rows));
        self.dispatch(Event::Draw);
    }

    pub fn dispatch(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.pop()
    }
}
