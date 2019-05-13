use crate::player::{MpvApi, Player, PlayerProperty};
use mpv::events::simple::Event;
use std::convert::From;

pub trait Poller {
    fn poll_events(&self);
}

impl<T> Poller for Player<T>
where
    T: MpvApi,
{
    fn poll_events(&self) {
        match self.mpv.wait_event(0.0) {
            Some(Ok(event)) => self.handle_event(event),
            _ => return,
        }
    }
    //
    //     fn handle_event(&self, event: Event) {
    //         match event {
    //             Event::PropertyChange { name, change, .. } => {
    //                 if let Ok(prop) = name.parse::<PlayerProperty>() {
    //                     self.ev_property_change(prop, change);
    //                 }
    //             }
    //             _ => return,
    //         }
    //     }
    //
    //     fn ev_property_change(&self, property: PlayerProperty, data: PropertyData) {
    //         for cb in self.cb_property_change {
    //             cb(property, data);
    //         }
    //     }
}

// impl<'a> From<Event<'a>> for PlayerEvent {
//     fn from(event: Event) -> Self {
//         match event {
//             Event::PropertyChange { name, change, .. } => {
//                 if let Ok(prop) = name.parse::<PlayerProperty>() {
//                     let value = prop.parse_property_data(change);
//                     PlayerEvent::PropertyChange(prop, value)
//                 } else {
//                     PlayerEvent::Unimplemented
//                 }
//             }
//             _ => PlayerEvent::Unimplemented,
//         }
//     }
// }
