use core::str::FromStr;
use mpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum PlayerProperty {
    Duration,
    Elapsed,
    Pause,
}

use PlayerProperty::*;

impl PlayerProperty {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Elapsed => "time-pos",
            Duration => "duration",
            Pause => "pause",
        }
    }

    pub(crate) fn player_format(&self) -> Format {
        match self {
            Elapsed => Format::Int64,
            Duration => Format::Int64,
            Pause => Format::String,
        }
    }
}

impl FromStr for PlayerProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<PlayerProperty, Self::Err> {
        match s {
            "time-pos" => Ok(Elapsed),
            "duration" => Ok(Duration),
            "pause" => Ok(Pause),
            _ => Err(()),
        }
    }
}
