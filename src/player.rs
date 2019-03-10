mod events;
mod mpv_api;
mod player_property;
mod seek_mode;
use cursive::views::TextContent;
pub use events::PlayerEventHandler;
pub use mpv_api::MpvApi;
pub use player_property::PlayerProperty;
use seek_mode::SeekMode;
use std::collections::HashMap;

pub struct Player<'a, T> {
    mpv: &'a T,
    text_contents: HashMap<PlayerProperty, TextContent>,
}

impl<'a, T> Player<'a, T>
where
    T: MpvApi,
{
    const PROPERTIES: [PlayerProperty; 2] = [PlayerProperty::Elapsed, PlayerProperty::Duration];
    pub fn new(mpv: &'a T) -> Player<'a, T> {
        let mut player = Player {
            mpv,
            text_contents: Default::default(),
        };

        player.init_defaults();
        player.observe_properties();
        player
    }

    pub fn play(&self, path: &str) {
        self.mpv.command("loadfile", &[path]).unwrap();
    }

    #[allow(dead_code)]
    pub fn append(&self, path: &str) {
        self.mpv.command("loadfile", &[path, "append"]).unwrap();
    }

    #[allow(dead_code)]
    pub fn elapsed(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Elapsed.as_str())
            .unwrap()
    }

    #[allow(dead_code)]
    pub fn seek(&self, seconds: i64, mode: SeekMode) {
        self.mpv
            .command("seek", &[&seconds.to_string(), mode.as_str()])
            .unwrap();
    }

    pub fn get_text_contents(&self) -> &HashMap<PlayerProperty, TextContent> {
        &self.text_contents
    }

    fn observe_properties(&mut self) {
        for property in Self::PROPERTIES.iter() {
            self.mpv
                .observe_property(property.as_str(), property.player_format(), 0)
                .unwrap();

            let text_content = TextContent::new(property.default_value());
            self.text_contents.insert(property.clone(), text_content);
        }
    }

    fn init_defaults(&self) {
        // Does what it says on the tin. Copied from the example.
        self.mpv.disable_deprecated_events().unwrap();

        // Playback will start when the cache has been filled up with this many
        // kilobytes of data (default: 0).
        self.mpv.set_property("cache-initial", 10).unwrap();

        // Prefetch next playlist entry while playback of the current entry is
        // ending (default: no). This merely opens the URL of the next playlist
        // entry as soon as the current URL is fully read.
        self.mpv.set_property("prefetch-playlist", "yes").unwrap();

        // Disable video output
        self.mpv.set_property("vo", "null").unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
