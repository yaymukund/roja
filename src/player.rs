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

pub type Metadata = HashMap<PlayerProperty, TextContent>;

pub struct Player<'a, T> {
    mpv: &'a T,
    metadata: Metadata,
}

impl<'a, T> Player<'a, T>
where
    T: MpvApi,
{
    const PROPERTIES: [PlayerProperty; 2] = [PlayerProperty::Elapsed, PlayerProperty::Duration];
    pub fn new(mpv: &'a T) -> Player<'a, T> {
        let mut player = Player {
            mpv,
            metadata: Default::default(),
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

    pub fn get_metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn observe_properties(&mut self) {
        for property in Self::PROPERTIES.iter() {
            self.mpv
                .observe_property(property.as_str(), property.player_format(), 0)
                .unwrap();

            let text_content = TextContent::new(property.default_value());
            self.metadata.insert(property.clone(), text_content);
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
    use mpv::Mpv;
    use mpv_api::{MockMpv, MpvCommand, MOCK_MP3};
    use serial_test_derive::serial;

    fn assert_did_invoke(mock_mpv: &MockMpv, mpv_command: MpvCommand) {
        assert!(mock_mpv
            .invocations()
            .iter()
            .any(|invocation| *invocation == mpv_command));
    }

    #[test]
    #[serial]
    fn test_new() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);

        Player::new(&mock_mpv);

        assert_did_invoke(&mock_mpv, MpvCommand::DisableDeprecatedEvents);
        assert_did_invoke(
            &mock_mpv,
            MpvCommand::SetProperty("cache-initial".to_string()),
        );
        assert_did_invoke(
            &mock_mpv,
            MpvCommand::SetProperty("prefetch-playlist".to_string()),
        );
        assert_did_invoke(&mock_mpv, MpvCommand::SetProperty("vo".to_string()));
    }

    #[test]
    #[serial]
    fn test_new_observes_properties() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);

        Player::new(&mock_mpv);

        for property in Player::<MockMpv>::PROPERTIES.iter() {
            assert_did_invoke(
                &mock_mpv,
                MpvCommand::ObserveProperty(property.as_str().to_string()),
            );
        }
    }

    #[test]
    #[serial]
    fn test_play() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);
        let player = Player::new(&mock_mpv);

        player.play(MOCK_MP3);
        assert_did_invoke(&mock_mpv, MpvCommand::Command("loadfile".to_string()));
    }

    #[test]
    #[serial]
    fn test_append() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);
        let player = Player::new(&mock_mpv);

        player.append(MOCK_MP3);
        assert_did_invoke(&mock_mpv, MpvCommand::Command("loadfile".to_string()));
    }

    #[test]
    #[serial]
    fn test_elapsed() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);
        let player = Player::new(&mock_mpv);
        player.play(MOCK_MP3);
        // wait for it to load.. would be good to not have to do this.
        mock_mpv.pause();
        player.elapsed();

        assert_did_invoke(
            &mock_mpv,
            MpvCommand::GetProperty(PlayerProperty::Elapsed.as_str().to_string()),
        );
    }

    #[test]
    #[serial]
    fn test_seek() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);
        let player = Player::new(&mock_mpv);
        player.play(MOCK_MP3);
        // wait for it to load.. would be good to not have to do this.
        mock_mpv.pause();
        player.seek(1, SeekMode::Absolute);
        assert_did_invoke(&mock_mpv, MpvCommand::Command("seek".to_string()));
    }

    #[test]
    #[serial]
    fn test_metadata_contains_observed_properties() {
        let mpv = Mpv::new().unwrap();
        let mock_mpv = MockMpv::new(&mpv);
        let player = Player::new(&mock_mpv);

        let metadata = player.get_metadata();
        for property in Player::<MockMpv>::PROPERTIES.iter() {
            assert!(metadata.contains_key(property));
        }
    }
}
