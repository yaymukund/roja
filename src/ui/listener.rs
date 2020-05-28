use super::Event;
use crate::util::channel;

pub trait Listener {
    fn on_event(&mut self, event: &Event);
}

pub trait IntoListener {
    type LType: Listener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType;
}

impl<L> IntoListener for L
where
    L: Listener,
{
    type LType = L;
    fn into_listener(self, _sender: channel::Sender<Event>) -> Self::LType {
        self
    }
}
