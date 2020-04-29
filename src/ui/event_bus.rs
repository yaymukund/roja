use super::{Event, State};

#[derive(Default)]
pub struct EventBus {
    listeners: Vec<Box<dyn Listener>>,
}

impl EventBus {
    pub fn register(&mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener);
    }

    pub fn dispatch(&self, event: Event, state: &mut State) {
        for listener in &self.listeners {
            listener.on_event(&event, state);
        }
    }

    pub fn wait_events(&self) -> impl Iterator<Item = Event> + '_ {
        self.listeners.iter().filter_map(|l| l.wait_event())
    }
}

pub trait Listener {
    fn on_event(&self, event: &Event, state: &mut State);
    fn wait_event(&self) -> Option<Event> {
        None
    }
}
