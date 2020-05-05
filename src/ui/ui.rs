use std::default::Default;
use std::io::{stdout, Write};

use super::{Event, EventBus, Listener, State};

#[derive(Default)]
pub struct UI {
    event_bus: EventBus,
    state: State,
}

impl UI {
    pub fn stopped(&self) -> bool {
        self.state.stopped()
    }

    pub fn draw(&mut self) {
        self.event_bus.dispatch(Event::Draw, &mut self.state);
    }

    pub fn tick(&mut self) {
        for ev in self.event_bus.wait_events() {
            self.event_bus.dispatch(ev, &mut self.state);
        }
        stdout().flush().expect("Could not flush");
    }

    pub fn register<D>(&mut self, data: D)
    where
        D: 'static + Listener,
    {
        self.event_bus.register(Box::new(data));
    }

    pub fn state(&self) -> &State {
        &self.state
    }
}
