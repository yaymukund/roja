use crate::player::PlayerProperty;
use crate::ui::{Event, IntoListener, Listener};
use crate::util::channel;

use libmpv::events::EventContext;

pub const PROPERTIES: [PlayerProperty; 7] = [
    PlayerProperty::Elapsed,
    PlayerProperty::DemuxerCacheState,
    PlayerProperty::Duration,
    PlayerProperty::Pause,
    PlayerProperty::MediaTitle,
    PlayerProperty::CoreIdle,
    PlayerProperty::PlaylistPos,
];

pub struct PlayerEventsListener<'a> {
    context: EventContext<'a>,
    sender: channel::Sender<Event>,
}

impl<'a> PlayerEventsListener<'a> {
    fn wait_event(&mut self) {
        if let Some(Ok(event)) = self.context.wait_event(0.0) {
            self.sender.send(event.into());
        }
    }
}

impl<'a> IntoListener for EventContext<'a> {
    type LType = PlayerEventsListener<'a>;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        self.disable_deprecated_events()
            .expect("could not disable deprecated events");

        for property in PROPERTIES.iter() {
            self.observe_property(property.as_str(), property.player_format(), 0)
                .expect("could not observe property");
        }

        Self::LType {
            context: self,
            sender,
        }
    }
}

impl<'a> Listener for PlayerEventsListener<'a> {
    fn on_event(&mut self, event: &Event) {
        if let Event::Tick = event {
            self.wait_event();
        }
    }
}
