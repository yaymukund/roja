use crate::player::PlayerProperty;
use crate::runtime::event::Event;
use mpv::events::simple::PropertyData;

pub type OnPropertyChange = dyn Fn(&PlayerProperty, &PropertyData<'_>);

#[derive(Default)]
pub struct EventHandler {
    property_change: Vec<Box<OnPropertyChange>>,
}

impl EventHandler {
    pub fn trigger(&self, event: Event<'_>) {
        if let Event::PropertyChange(player_property, property_data) = event {
            self.trigger_property_change(player_property, property_data)
        }
    }
    pub fn on_property_change(&mut self, callback: Box<OnPropertyChange>) {
        self.property_change.push(callback);
    }

    fn trigger_property_change(
        &self,
        player_property: PlayerProperty,
        property_data: PropertyData<'_>,
    ) {
        for callback in &self.property_change {
            callback(&player_property, &property_data);
        }
    }
}
