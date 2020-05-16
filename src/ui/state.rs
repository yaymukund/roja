use super::Event;
use super::Layout;
use crate::util::terminal;
use std::collections::VecDeque;

pub struct State {
    events: VecDeque<Event>,
    stopped: bool,
    layout: Layout,
}

impl State {
    pub fn new() -> Self {
        let (cols, rows) = terminal::size();
        State {
            events: VecDeque::new(),
            stopped: false,
            layout: Layout::new(cols, rows),
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.layout = Layout::new(cols, rows);
        self.dispatch(Event::ResizeListener(self.layout.clone()));
        self.dispatch(Event::Draw);
    }

    pub fn dispatch(&mut self, event: Event) {
        self.events.push_back(event);
    }

    pub fn next_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }
}
