use super::{Event, EventBus, IntoListener, Listener, State};
use crate::util::terminal;

pub struct UI {
    event_bus: EventBus,
    state: State,
}

impl UI {
    pub fn new() -> Self {
        Self {
            event_bus: EventBus::new(),
            state: State::new(),
        }
    }
    pub fn stopped(&self) -> bool {
        self.state.stopped()
    }

    pub fn tick(&mut self) {
        self.dispatch(Event::Tick);

        while let Some(event) = self.state.next_event() {
            self.dispatch(event);
        }

        terminal::flush();
    }

    fn dispatch(&mut self, event: Event) {
        self.event_bus.dispatch(event, &mut self.state);
    }

    pub fn register<D>(&mut self, data: D)
    where
        D: 'static + IntoListener,
    {
        let mut listener = data.into_listener(self.state.layout());
        listener.on_event(&Event::Draw, &mut self.state);
        self.event_bus.register(Box::new(listener));
    }
}
