use core::str::FromStr;
use mpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlayerProperty {
    Duration,
    Elapsed,
    Pause,
    MediaTitle,
}

use PlayerProperty::*;

impl PlayerProperty {
    pub fn as_str(&self) -> &str {
        match self {
            Elapsed => "time-pos",
            Duration => "duration",
            Pause => "pause",
            MediaTitle => "media-title",
        }
    }

    pub fn player_format(&self) -> Format {
        match self {
            Elapsed => Format::Int64,
            Duration => Format::Int64,
            Pause => Format::String,
            MediaTitle => Format::String,
        }
    }
}

impl FromStr for PlayerProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "time-pos" => Ok(Elapsed),
            "duration" => Ok(Duration),
            "pause" => Ok(Pause),
            "media-title" => Ok(MediaTitle),
            _ => Err(()),
        }
    }
}
