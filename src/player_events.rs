mod handle_player_event;

pub use handle_player_event::handle_player_event;

use crate::player::PlayerProperty;
use mpv::events::simple::Event as MpvEvent;
use mpv::events::simple::PropertyData;

pub enum PlayerEvent<'a> {
    PropertyChange(PlayerProperty, PropertyData<'a>),
    Unimplemented,
}

impl<'a> From<MpvEvent<'a>> for PlayerEvent<'a> {
    fn from(event: MpvEvent<'_>) -> PlayerEvent<'_> {
        match event {
            MpvEvent::PropertyChange { name, change, .. } => {
                if let Ok(prop) = name.parse::<PlayerProperty>() {
                    PlayerEvent::PropertyChange(prop, change)
                } else {
                    PlayerEvent::Unimplemented
                }
            }
            _ => PlayerEvent::Unimplemented,
        }
    }
}
