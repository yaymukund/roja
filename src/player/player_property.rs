use crate::util::format_duration;
use core::str::FromStr;
use mpv::events::simple::PropertyData;
use mpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlayerProperty {
    Duration,
    Elapsed,
    Pause,
}

use PlayerProperty::*;

impl PlayerProperty {
    pub fn as_str(&self) -> &str {
        match self {
            Elapsed => "time-pos",
            Duration => "duration",
            Pause => "pause",
        }
    }

    pub fn player_format(&self) -> Format {
        match self {
            Elapsed => Format::Int64,
            Duration => Format::Int64,
            Pause => Format::String,
        }
    }

    pub fn parse_property_data(&self, property_data: &PropertyData<'_>) -> String {
        match (self, property_data) {
            (Elapsed, PropertyData::Int64(data)) => format_duration(*data),
            (Duration, PropertyData::Int64(data)) => format_duration(*data),
            (_, _) => String::from("Error: Unknown Property"),
        }
    }
}

impl FromStr for PlayerProperty {
    type Err = ();

    fn from_str(s: &str) -> Result<PlayerProperty, Self::Err> {
        match s {
            "time-pos" => Ok(PlayerProperty::Elapsed),
            "duration" => Ok(PlayerProperty::Duration),
            _ => Err(()),
        }
    }
}
