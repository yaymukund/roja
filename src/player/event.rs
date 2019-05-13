use crate::player::PlayerProperty;
use mpv::events::simple::Event as MpvEvent;
use mpv::events::simple::PropertyData;

pub enum Event<'a> {
    PropertyChange(PlayerProperty, PropertyData<'a>),
    Unimplemented,
}

impl<'a> From<MpvEvent<'a>> for Event<'a> {
    fn from(event: MpvEvent) -> Event {
        match event {
            MpvEvent::PropertyChange { name, change, .. } => {
                if let Ok(prop) = name.parse::<PlayerProperty>() {
                    Event::PropertyChange(prop, change)
                } else {
                    Event::Unimplemented
                }
            }
            _ => Event::Unimplemented,
        }
    }
}
