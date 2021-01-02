mod event;
mod label;
pub mod layout;
mod listener;

use crate::util::{channel, terminal};
pub use event::*;
pub use label::Label;
pub use listener::{IntoListener, Listener};

pub struct QuitError;

pub struct UI {
    listeners: Vec<Box<dyn Listener>>,
    receiver: channel::Receiver<Event>,
    sender: channel::Sender<Event>,
}

impl UI {
    pub fn new() -> Self {
        let (sender, receiver) = channel::unbounded();
        Self {
            listeners: Vec::new(),
            receiver,
            sender,
        }
    }

    pub fn tick(&mut self) -> Result<(), QuitError> {
        self.broadcast(&Event::Tick);

        loop {
            match self.receiver.try_recv() {
                Ok(Event::Quit) => return Err(QuitError),
                Ok(event) => self.broadcast(&event),
                Err(channel::TryRecvError::Disconnected) => panic!("disconnected before quitting"),
                Err(channel::TryRecvError::Empty) => {
                    terminal::flush();
                    return Ok(());
                }
            }
        }
    }

    pub fn redraw(&mut self) {
        let (width, height) = terminal::size();
        self.broadcast(&Event::Resize(width, height));
    }

    fn broadcast(&mut self, event: &Event) {
        for listener in &mut self.listeners {
            listener.on_event(event)
        }
    }

    pub fn register<D>(&mut self, data: D)
    where
        D: 'static + IntoListener,
    {
        let listener = data.into_listener(self.sender.clone());
        self.listeners.push(Box::new(listener));
    }
}
