use mpv::events::simple::Event as MpvEvent;
use mpv::Mpv;

use super::player_property::PlayerProperty;

pub const PROPERTIES: [PlayerProperty; 4] = [
    PlayerProperty::Elapsed,
    PlayerProperty::Duration,
    PlayerProperty::Pause,
    PlayerProperty::MediaTitle,
];

pub struct Player {
    mpv: Mpv,
}

impl Player {
    pub fn new() -> Self {
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
            .unwrap_or(0_i64)
    }

    pub fn duration(&self) -> i64 {
        self.mpv
            .get_property(PlayerProperty::Duration.as_str())
            .unwrap_or(1_i64)
    }

    pub fn percent_complete(&self) -> u16 {
        let elapsed = self.elapsed() as f64;
        let duration = self.duration() as f64;
        let percent_complete = elapsed / duration * 100.0;
        if percent_complete > 0.0 {
            percent_complete as u16
        } else {
            0
        }
    }

    pub fn seek_forward(&self) {
        self.mpv.seek_forward(5.0).expect("couldn't seek forward");
    }

    pub fn seek_backward(&self) {
        self.mpv.seek_backward(5.0).expect("couldn't seek backward");
    }

    pub fn paused(&self) -> bool {
        let pause: String = self.mpv.get_property("pause").unwrap();
        pause == "yes"
    }

    pub fn title(&self) -> String {
        self.mpv
            .get_property("media-title")
            .unwrap_or_else(|_| String::from(""))
    }

    pub fn artist(&self) -> String {
        self.mpv
            .get_property("metadata/by-key/artist")
            .unwrap_or_else(|_| String::from(""))
    }

    pub fn idle_active(&self) -> bool {
        let idle_active: String = self.mpv.get_property("idle-active").unwrap();
        idle_active == "yes"
    }

    pub fn toggle_pause(&self) {
        let next = if self.paused() { "no" } else { "yes" };
        self.mpv.set_property("pause", next).unwrap();
    }

    pub fn wait_event(&self) -> Option<MpvEvent<'_>> {
        if let Some(Ok(event)) = unsafe { self.mpv.wait_event(0.0) } {
            Some(event)
        } else {
            None
        }
    }

    fn command(&self, name: &str, args: &[&str]) {
        self.mpv
            .command(name, args)
            .unwrap_or_else(|e| println!("mpv {} error: {:?}", name, e));
    }
}
