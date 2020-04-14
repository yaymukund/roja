mod player_property;
mod seek_mode;

use mpv::events::simple::Event as MpvEvent;
pub(crate) use player_property::PlayerProperty;
use seek_mode::SeekMode;

pub(crate) const PROPERTIES: [PlayerProperty; 3] = [
    PlayerProperty::Elapsed,
    PlayerProperty::Duration,
    PlayerProperty::Pause,
];

use mpv::Mpv;

pub(crate) struct Player {
    mpv: Mpv,
}

impl Player {
    pub(crate) fn new() -> Player {
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

    pub(crate) fn play(&self, path: &str) {
        self.command("loadfile", &[path]);
    }

    #[allow(dead_code)]
    pub(crate) fn append(&self, path: &str) {
        self.command("loadfile", &[path, "append"]);
    }

    pub(crate) fn elapsed(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Elapsed.as_str())
            .unwrap()
    }

    pub(crate) fn duration(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Duration.as_str())
            .unwrap()
    }

    pub(crate) fn percent_complete(&self) -> u16 {
        let elapsed = self.elapsed() as f64;
        let duration = self.duration() as f64;
        let percent_complete = elapsed / duration * 100.0;
        if percent_complete > 0.0 {
            percent_complete as u16
        } else {
            0
        }
    }

    pub(crate) fn seek(&self, seconds: i64, mode: SeekMode) {
        self.command("seek", &[&seconds.to_string(), mode.as_str()]);
    }

    pub(crate) fn seek_forward(&self) {
        self.seek(5, SeekMode::Relative);
    }

    pub(crate) fn seek_backward(&self) {
        self.seek(-5, SeekMode::Relative);
    }

    pub(crate) fn paused(&self) -> bool {
        let pause: String = self.mpv.get_property("pause").unwrap();
        pause == "yes"
    }

    pub(crate) fn idle_active(&self) -> bool {
        let idle_active: String = self.mpv.get_property("idle-active").unwrap();
        idle_active == "yes"
    }

    pub(crate) fn toggle_pause(&self) {
        let next = if self.paused() { "no" } else { "yes" };
        self.mpv.set_property("pause", next).unwrap();
    }

    pub(crate) fn poll_event(&self) -> Option<MpvEvent> {
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
