mod mpv_api;
mod player_property;
mod seek_mode;
use mpv::events::simple::Event as MpvEvent;
pub use mpv_api::MpvApi;
pub use player_property::PlayerProperty;
use seek_mode::SeekMode;

pub const PROPERTIES: [PlayerProperty; 3] = [
    PlayerProperty::Elapsed,
    PlayerProperty::Duration,
    PlayerProperty::Pause,
];

use mpv::Mpv;
use std::cell::RefCell;
use std::rc::Rc;
pub type RcPlayer = Rc<RefCell<Player<Mpv>>>;

pub struct Player<T> {
    mpv: T,
}

impl<T> Player<T>
where
    T: MpvApi,
{
    pub fn new(mpv: T) -> Player<T> {
        let mut player = Player { mpv };

        player.init_defaults();
        player.observe_properties();
        player
    }

    pub fn play(&self, path: &str) {
        self.command("loadfile", &[path]);
    }

    #[allow(dead_code)]
    pub fn append(&self, path: &str) {
        self.command("loadfile", &[path, "append"]);
    }

    #[allow(dead_code)]
    pub fn elapsed(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Elapsed.as_str())
            .unwrap()
    }

    pub fn seek(&self, seconds: i64, mode: SeekMode) {
        self.command("seek", &[&seconds.to_string(), mode.as_str()]);
    }

    pub fn seek_forward(&self) {
        self.seek(5, SeekMode::Relative);
    }

    pub fn seek_backward(&self) {
        self.seek(-5, SeekMode::Relative);
    }

    pub fn paused(&self) -> bool {
        let paused: String = self.mpv.get_property("pause").unwrap();
        paused == "yes"
    }

    pub fn core_idle(&self) -> bool {
        let core_idle: String = self.mpv.get_property("core-idle").unwrap();
        core_idle == "yes"
    }

    pub fn toggle_pause(&self) {
        let next = if self.paused() { "no" } else { "yes" };
        self.mpv.set_property("pause", next).unwrap();
    }

    pub fn poll_events(&self) -> Option<MpvEvent> {
        if let Some(Ok(event)) = self.mpv.wait_event(0.0) {
            Some(event)
        } else {
            None
        }
    }

    fn command(&self, name: &str, args: &[&str]) {
        self.mpv
            .command(name, args)
            .unwrap_or_else(|e| log::error!("mpv {} error: {:?}", name, e));
    }

    /// tells the mpv api that we want property change events.
    fn observe_properties(&mut self) {
        for property in PROPERTIES.iter() {
            self.mpv
                .observe_property(property.as_str(), property.player_format(), 0)
                .unwrap();
        }
    }

    fn init_defaults(&self) {
        // Does what it says on the tin. Copied from the example.
        self.mpv.disable_deprecated_events().unwrap();

        // Playback will start when the cache has been filled up with this many
        // kilobytes of data (default: 0).
        self.mpv.set_property("cache-secs", 10).unwrap();

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
    use mpv_api::{MockMpv, MpvCommand, MOCK_MP3};
    use serial_test_derive::serial;
    use std::{thread, time};

    pub fn pause_test() {
        let ms = time::Duration::from_millis(100);
        thread::sleep(ms);
    }

    trait Assertable {
        fn assert_did_invoke(&self, mpv_command: MpvCommand);
    }

    impl Assertable for Player<MockMpv> {
        fn assert_did_invoke(&self, mpv_command: MpvCommand) {
            assert!(self
                .mpv
                .invocations()
                .iter()
                .any(|invocation| *invocation == mpv_command));
        }
    }

    #[test]
    #[serial]
    fn test_new() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);

        player.assert_did_invoke(MpvCommand::DisableDeprecatedEvents);
        player.assert_did_invoke(MpvCommand::SetProperty("cache-initial".to_string()));
        player.assert_did_invoke(MpvCommand::SetProperty("prefetch-playlist".to_string()));
        player.assert_did_invoke(MpvCommand::SetProperty("vo".to_string()));
    }

    #[test]
    #[serial]
    fn test_new_observes_properties() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);

        for property in PROPERTIES.iter() {
            player.assert_did_invoke(MpvCommand::ObserveProperty(property.as_str().to_string()));
        }
    }

    #[test]
    #[serial]
    fn test_play() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);

        player.play(MOCK_MP3);
        player.assert_did_invoke(MpvCommand::Command("loadfile".to_string()));
    }

    #[test]
    #[serial]
    fn test_append() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);

        player.append(MOCK_MP3);
        player.assert_did_invoke(MpvCommand::Command("loadfile".to_string()));
    }

    #[test]
    #[serial]
    fn test_elapsed() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);
        player.play(MOCK_MP3);
        // wait for it to load.. would be good to not have to do this.
        pause_test();
        player.elapsed();

        player.assert_did_invoke(MpvCommand::GetProperty(
            PlayerProperty::Elapsed.as_str().to_string(),
        ));
    }

    #[test]
    #[serial]
    fn test_seek() {
        let mock_mpv = MockMpv::new();
        let player = Player::new(mock_mpv);

        player.play(MOCK_MP3);
        // wait for it to load.. would be good to not have to do this.
        pause_test();
        player.seek(1, SeekMode::Absolute);
        player.assert_did_invoke(MpvCommand::Command("seek".to_string()));
    }
}
