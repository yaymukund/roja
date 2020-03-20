pub const LABELS: [Label; 2] = [Label::TrackTime, Label::PlayPauseIndicator];

/// A label is a name for a place in the UI where we render dynamic content. Dynamic, in this
/// context, means that the UI needs to update when the content changes.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Label {
    TrackTime,
    PlayPauseIndicator,
}

impl Label {
    pub fn default_value(&self) -> &str {
        match self {
            TrackTime => "",
            PlayPauseIndicator => "",
        }
    }
}
