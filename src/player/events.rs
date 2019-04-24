use crate::player::{MpvApi, Player, PlayerProperty};
use mpv::events::simple::Event;
use std::convert::From;

pub trait PlayerEventHandler {
    fn poll_events(&mut self);
    fn wait_event(&self) -> Option<PlayerEvent>;
}

impl<T> PlayerEventHandler for Player<T>
where
    T: MpvApi,
{
    fn poll_events(&mut self) {
        match self.wait_event() {
            Some(PlayerEvent::PropertyChange(prop, value)) => {
                let metadata = self.metadata.get_mut(&prop).unwrap();
                metadata.set_content(value);
            }
            _ => return,
        }
    }

    fn wait_event(&self) -> Option<PlayerEvent> {
        match self.mpv.wait_event(0.0) {
            Some(Ok(event)) => Some(PlayerEvent::from(event)),
            _ => None,
        }
    }
}

impl<'a> From<Event<'a>> for PlayerEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::PropertyChange { name, change, .. } => {
                if let Ok(prop) = name.parse::<PlayerProperty>() {
                    let value = prop.parse_property_data(change);
                    PlayerEvent::PropertyChange(prop, value)
                } else {
                    PlayerEvent::Unimplemented
                }
            }
            _ => PlayerEvent::Unimplemented,
        }
    }
}

pub enum PlayerEvent {
    PropertyChange(PlayerProperty, String),
    Unimplemented,
}
