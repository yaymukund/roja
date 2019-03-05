mod player_property;
mod seek_mode;
use cursive::views::TextContent;
use mpv::events::events_simple::{Event, PropertyData};
use mpv::Mpv;
pub use player_property::PlayerProperty;
use seek_mode::SeekMode;
use std::collections::HashMap;

pub struct Player {
    mpv: Mpv,
    text_contents: HashMap<PlayerProperty, TextContent>,
}

impl Player {
    const PROPERTIES: [PlayerProperty; 1] = [PlayerProperty::Elapsed];
    pub fn new() -> Player {
        let mpv = Mpv::new().unwrap();

        // Does what it says on the tin. Copied from the example.
        mpv.disable_deprecated_events().unwrap();

        // Playback will start when the cache has been filled up with this many
        // kilobytes of data (default: 0).
        mpv.set_property("cache-initial", 10).unwrap();

        // Prefetch next playlist entry while playback of the current entry is
        // ending (default: no). This merely opens the URL of the next playlist
        // entry as soon as the current URL is fully read.
        mpv.set_property("prefetch-playlist", "yes").unwrap();

        // Disable video output
        mpv.set_property("vo", "null").unwrap();

        Player {
            mpv,
            text_contents: Default::default(),
        }
    }

    pub fn play(&self, path: &str) {
        self.mpv.command("loadfile", &[path]).unwrap();
    }

    pub fn append(&self, path: &str) {
        self.mpv.command("loadfile", &[path, "append"]).unwrap();
    }

    pub fn elapsed(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Elapsed.as_str())
            .unwrap()
    }

    pub fn seek(&self, seconds: i64, mode: SeekMode) {
        self.mpv
            .command("seek", &[&seconds.to_string(), mode.as_str()])
            .unwrap();
    }

    pub fn text_contents(&mut self) -> &HashMap<PlayerProperty, TextContent> {
        for property in Player::PROPERTIES.iter() {
            self.mpv
                .observe_property(property.as_str(), property.player_format(), 0)
                .unwrap();

            let text_content = TextContent::new("");
            self.text_contents.insert(property.clone(), text_content);
        }

        &self.text_contents
    }

    pub fn poll_events(&mut self) {
        let event = unsafe { self.mpv.wait_event(0.0) };

        if event.is_none() {
            return;
        }

        let event = event.unwrap();

        if event.is_err() {
            return;
        }

        let event = event.unwrap();

        if let Event::PropertyChange {
            name,
            change: PropertyData::Int64(data),
            reply_userdata: _userdata,
        } = event
        {
            println!("property change found!");
            if name == "time-pos" {
                self.text_contents
                    .entry(PlayerProperty::Elapsed)
                    .and_modify(|tc| tc.set_content(data.to_string()));
            }
        }
    }
}
