mod player_property;

use std::fmt::Display;
use std::path::PathBuf;

use log::debug;
use mpv::events::simple::Event as MpvEvent;
use mpv::Mpv;

use crate::SETTINGS;
use player_property::PlayerProperty;

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
        mpv.set_property("demuxer-max-bytes", "100MiB").unwrap();

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

    pub fn play(&self, path: &PathBuf) {
        let path = full_path(path);
        self.command("loadfile", &[&path]);

        if self.paused() {
            self.toggle_pause();
        }
    }

    pub fn append(&self, path: &PathBuf) {
        let path = full_path(path);
        self.command("loadfile", &[&path, "append"]);
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

        if percent_complete > 100.0 {
            100
        } else if percent_complete > 0.0 {
            percent_complete as u16
        } else {
            0
        }
    }

    pub fn seek_forward(&self) {
        self.command("seek", &["5.0", "relative"]);
    }

    pub fn seek_backward(&self) {
        self.command("seek", &["-5.0", "relative"]);
    }

    pub fn toggle_pause(&self) {
        let next = if self.paused() { "no" } else { "yes" };
        self.mpv.set_property("pause", next).unwrap();
    }

    pub fn paused(&self) -> bool {
        let pause: String = self.mpv.get_property("pause").unwrap();
        pause == "yes"
    }

    /// track title from the file metadata
    pub fn title(&self) -> String {
        self.mpv
            .get_property("media-title")
            .unwrap_or_else(|_| String::from(""))
    }

    /// track artist from the file metadata
    pub fn artist(&self) -> String {
        self.mpv
            .get_property("metadata/by-key/artist")
            .unwrap_or_else(|_| String::from(""))
    }

    /// returns if the mpv handle has loaded a track
    pub fn is_track_loaded(&self) -> bool {
        let idle: bool = self.mpv.get_property("idle-active").unwrap();
        !idle
    }

    pub fn wait_event(&self) -> Option<MpvEvent<'_>> {
        if let Some(Ok(event)) = unsafe { self.mpv.wait_event(0.0) } {
            Some(event)
        } else {
            None
        }
    }

    fn command<T>(&self, name: T, args: &[&str])
    where
        T: AsRef<str> + Display,
    {
        self.mpv
            .command(name.as_ref(), args)
            .unwrap_or_else(|e| debug!("mpv {} error: {:?}", name, e));
    }
}

fn full_path(path: &PathBuf) -> String {
    let path = SETTINGS.with(|s| s.music_library_path().join(path));
    format!(
        "\"{}\"",
        path.to_str().expect("could not convert path to str")
    )
}
