use super::{Event, IntoListener, Listener};
use crate::util::{channel, terminal};

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
        self.send(&Event::Tick);

        loop {
            match self.receiver.recv() {
                Some(Event::Quit) => return Err(QuitError),
                Some(event) => self.send(&event),
                None => {
                    terminal::flush();
                    return Ok(());
                }
            }
        }
    }

    pub fn first_draw(&mut self) {
        let (width, height) = terminal::size();
        self.send(&Event::Resize(width, height));
    }

    fn send(&mut self, event: &Event) {
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
