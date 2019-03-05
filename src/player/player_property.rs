use mpv::Format;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum PlayerProperty {
    Elapsed,
}

impl PlayerProperty {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerProperty::Elapsed => "time-pos",
        }
    }

    pub fn player_format(&self) -> Format {
        match self {
            PlayerProperty::Elapsed => Format::Int64,
        }
    }
}
