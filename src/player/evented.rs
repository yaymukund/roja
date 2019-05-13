use crate::player::event_handler::OnPropertyChange;
use crate::player::{MpvApi, Player};

pub trait Evented {
    fn on_property_change(&mut self, callback: Box<OnPropertyChange>);
}

impl<T> Evented for Player<T>
where
    T: MpvApi,
{
    fn on_property_change(&mut self, callback: Box<OnPropertyChange>) {
        self.event_handler.on_property_change(callback);
    }
}
