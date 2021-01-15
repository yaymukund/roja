use core::str::FromStr;
use libmpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlayerProperty {
    CoreIdle,
    DemuxerCacheState,
    Duration,
    Elapsed,
    MediaTitle,
    Pause,
    PlaylistPos,
}

use PlayerProperty::*;

impl PlayerProperty {
    pub fn as_str(&self) -> &str {
        match self {
            CoreIdle => "core-idle",
            DemuxerCacheState => "demuxer-cache-state",
            Elapsed => "time-pos",
            Duration => "duration",
            MediaTitle => "media-title",
            Pause => "pause",
            PlaylistPos => "playlist-pos",
        }
    }

    pub fn player_format(&self) -> Format {
        match self {
            CoreIdle => Format::String,
            DemuxerCacheState => Format::Node,
            Elapsed => Format::Int64,
            Duration => Format::Int64,
            MediaTitle => Format::String,
            Pause => Format::String,
            PlaylistPos => Format::Int64,
        }
    }
}

impl FromStr for PlayerProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "core-idle" => Ok(CoreIdle),
            "demuxer-cache-state" => Ok(DemuxerCacheState),
            "time-pos" => Ok(Elapsed),
            "duration" => Ok(Duration),
            "media-title" => Ok(MediaTitle),
            "pause" => Ok(Pause),
            "playlist-pos" => Ok(PlaylistPos),
            _ => Err(()),
        }
    }
}
