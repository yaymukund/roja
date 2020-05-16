use super::{Event, State};

#[derive(Default)]
pub struct EventBus {
    listeners: Vec<Box<dyn Listener>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn register(&mut self, listener: Box<dyn Listener>) {
        self.listeners.push(listener);
    }

    pub fn dispatch(&mut self, event: Event, state: &mut State) {
        log::info!("Dispatching event: {:?}", event);
        for listener in &mut self.listeners {
            listener.on_event(&event, state);
        }
    }
}

pub trait Listener {
    fn on_event(&mut self, event: &Event, state: &mut State);
}

pub trait IntoListener {
    type LType: Listener;
    fn into_listener(self, cols: u16, rows: u16) -> Self::LType;
}

impl<L> IntoListener for L
where
    L: Listener,
{
    type LType = L;
    fn into_listener(self, _cols: u16, _rows: u16) -> Self::LType {
        self
    }
}
