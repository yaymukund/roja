use crate::player::Player;
use crossbeam_utils::thread;
use cursive::views::TextContent;
use mpv::events::events_simple::{Event, PropertyData};
use mpv::Format;
use std::collections::HashMap;

pub trait HasMetadata {
    fn metadata(&self) -> HashMap<String, TextContent>;
    fn observe_property(&self, name: &str, format: Format) -> ();
}

impl HasMetadata for Player {
    fn observe_property(&self, name: &str, format: Format) {
        self.mpv.observe_property(name, format, 0).unwrap();
    }

    fn metadata(&self) -> HashMap<String, TextContent> {
        self.mpv
            .observe_property("time-pos", Format::Int64, 0)
            .unwrap();

        let mut metadata = HashMap::new();
        let mut elapsed = TextContent::new("");

        thread::scope(|s| {
            s.spawn(|_| loop {
                let event = unsafe { self.mpv.wait_event(600.) };
                if let Some(Ok(Event::PropertyChange {
                    change: PropertyData::Int64(data),
                    ..
                })) = event
                {
                    println!("Now the time elapsed is {}", data.to_string());
                    elapsed.set_content(data.to_string());
                }
            });
        });
        println!("Finished the crossbean thread");

        metadata.insert("elapsed".to_string(), elapsed.clone());
        metadata
    }
}
