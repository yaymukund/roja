use crate::player::{MpvApi, Player, PlayerProperty};
use mpv::events::simple::{Event, PropertyData};

pub trait PlayerEventHandler {
    fn poll_events(&mut self);
    fn handle_event(&mut self, event: Event);

    // Event Types
    fn ev_property_change(&mut self, name: &str, change: PropertyData);
}

impl<'a, T> PlayerEventHandler for Player<'a, T>
where
    T: MpvApi,
{
    fn poll_events(&mut self) {
        let event = { self.mpv.wait_event(0.0) };

        if let Some(Ok(event)) = event {
            self.handle_event(event);
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::PropertyChange { name, change, .. } => self.ev_property_change(name, change),
            _ => return,
        }
    }

    fn ev_property_change(&mut self, name: &str, data: PropertyData) {
        if let Ok(prop) = name.parse::<PlayerProperty>() {
            let new_value = prop.parse_property_data(data);

            let metadata = self.metadata.get_mut(&prop).unwrap();
            metadata.set_content(new_value);
        }
    }
}
