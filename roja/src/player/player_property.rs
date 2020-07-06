use core::str::FromStr;
use libmpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlayerProperty {
    CoreIdle,
    Duration,
    Elapsed,
    MediaTitle,
    Pause,
}

use PlayerProperty::*;

impl PlayerProperty {
    pub fn as_str(&self) -> &str {
        match self {
            CoreIdle => "core-idle",
            Elapsed => "time-pos",
            Duration => "duration",
            MediaTitle => "media-title",
            Pause => "pause",
        }
    }

    pub fn player_format(&self) -> Format {
        match self {
            CoreIdle => Format::String,
            Elapsed => Format::Int64,
            Duration => Format::Int64,
            MediaTitle => Format::String,
            Pause => Format::String,
        }
    }
}

impl FromStr for PlayerProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "core-idle" => Ok(CoreIdle),
            "time-pos" => Ok(Elapsed),
            "duration" => Ok(Duration),
            "media-title" => Ok(MediaTitle),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}
