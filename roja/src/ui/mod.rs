mod event;
mod label;
pub mod layout;
mod listener;

use crate::util::{channel, terminal};
use anyhow::Result;
pub use event::*;
pub use label::Label;
pub use listener::{IntoListener, Listener};

pub struct UI {
    listeners: Vec<Box<dyn Listener>>,
    receiver: channel::Receiver<Event>,
    sender: channel::Sender<Event>,
}

pub enum Loop {
    Continue,
    Stop,
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

    pub fn tick(&mut self) -> Result<Loop> {
        self.broadcast(&Event::Tick)?;

        loop {
            match self.receiver.try_recv() {
                Ok(Event::Quit) => return Ok(Loop::Stop),
                Ok(event) => self.broadcast(&event)?,
                Err(channel::TryRecvError::Disconnected) => {
                    panic!("disconnected before quitting");
                }
                Err(channel::TryRecvError::Empty) => {
                    terminal::flush();
                    return Ok(Loop::Continue);
                }
            }
        }
    }

    pub fn redraw(&mut self) -> Result<()> {
        let (width, height) = terminal::size();
        self.broadcast(&Event::Resize(width, height))
    }

    fn broadcast(&mut self, event: &Event) -> Result<()> {
        for listener in &mut self.listeners {
            listener.on_event(event)?
        }

        Ok(())
    }

    pub fn register<D>(&mut self, data: D)
    where
        D: 'static + IntoListener,
    {
        let listener = data.into_listener(self.sender.clone());
        self.listeners.push(Box::new(listener));
    }
}
