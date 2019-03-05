pub mod has_metadata;
use mpv::Mpv;

pub struct Player {
    mpv: Mpv,
}

impl Player {
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

        Player { mpv }
    }

    pub fn play(&self, path: &str) {
        self.mpv.command("loadfile", &[path]).unwrap();
    }

    pub fn append(&self, path: &str) {
        self.mpv.command("loadfile", &[path, "append"]).unwrap();
    }

    pub fn elapsed(&self) -> i64 {
        self.mpv.get_property("time-pos").unwrap()
    }

    pub fn seek(&self, seconds: i64, mode: SeekMode) {
        self.mpv
            .command("seek", &[&seconds.to_string(), mode.as_str()])
            .unwrap();
    }
}

pub enum SeekMode {
    // Relative,
    Absolute,
    // AbsolutePercent,
    // RelativePercent,
    // Keyframes,
    // Exact,
}

impl SeekMode {
    pub fn as_str(&self) -> &str {
        match self {
            // SeekMode::Relative => "relative",
            SeekMode::Absolute => "absolute",
            // SeekMode::AbsolutePercent => "absolute-percent",
            // SeekMode::RelativePercent => "relative-percent",
            // SeekMode::Keyframes => "keyframes",
            // SeekMode::Exact => "exact",
        }
    }
}
