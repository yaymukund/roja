mod player_property;
mod seek_mode;

use mpv::events::simple::Event as MpvEvent;
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

pub type RcPlayer = Rc<RefCell<Player>>;

pub struct Player {
    mpv: Mpv,
}

impl Player {
    pub fn new() -> Player {
        let mpv = Mpv::new().expect("could not initialize mpv instance");

        // Does what it says on the tin. Copied from the example.
        mpv.disable_deprecated_events().unwrap();

        // Playback will start when the cache has been filled up with this many
        // kilobytes of data (default: 0).
        mpv.set_property("cache-secs", 10).unwrap();

        // Prefetch next playlist entry while playback of the current entry is
        // ending (default: no). This merely opens the URL of the next playlist
        // entry as soon as the current URL is fully read.
        mpv.set_property("prefetch-playlist", "yes").unwrap();

        // Disable video output
        mpv.set_property("vo", "null").unwrap();

        // tells the mpv api that we want property change events.
        for property in PROPERTIES.iter() {
            mpv.observe_property(property.as_str(), property.player_format(), 0)
                .unwrap();
        }

        Player { mpv }
    }

    pub fn play(&self, path: &str) {
        self.command("loadfile", &[path]);
    }

    #[allow(dead_code)]
    pub fn append(&self, path: &str) {
        self.command("loadfile", &[path, "append"]);
    }

    pub fn elapsed(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Elapsed.as_str())
            .unwrap()
    }

    pub fn duration(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Duration.as_str())
            .unwrap()
    }

    pub fn percent_complete(&self) -> usize {
        let elapsed = self.elapsed() as f64;
        let duration = self.duration() as f64;
        (elapsed / duration * 100.0) as usize
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
        if let Some(Ok(event)) = unsafe { self.mpv.wait_event(0.0) } {
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
}
