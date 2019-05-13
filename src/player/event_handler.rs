use super::event::Event;
use crate::player::PlayerProperty;
use mpv::events::simple::PropertyData;

pub type OnPropertyChange = Fn(&PlayerProperty, &PropertyData);

#[derive(Default)]
pub struct EventHandler {
    property_change: Vec<Box<OnPropertyChange>>,
}

impl EventHandler {
    pub fn trigger(&self, event: Event) {
        match event {
            Event::PropertyChange(player_property, property_data) => {
                self.trigger_property_change(player_property, property_data)
            }
            _ => return,
        }
    }
    pub fn on_property_change(&mut self, callback: Box<OnPropertyChange>) {
        self.property_change.push(callback);
    }

    fn trigger_property_change(
        &self,
        player_property: PlayerProperty,
        property_data: PropertyData,
    ) {
        for callback in &self.property_change {
            callback(&player_property, &property_data);
        }
    }
}
