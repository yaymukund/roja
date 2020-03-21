pub const LABELS: [Label; 3] = [
    Label::ElapsedTime,
    Label::PlayPauseIndicator,
    Label::TotalTime,
];

/// A label is a name for a place in the UI where we render dynamic content. Dynamic, in this
/// context, means that the UI needs to update when the content changes.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Label {
    ElapsedTime,
    PlayPauseIndicator,
    TotalTime,
}

use Label::*;
impl Label {
    pub fn default_value(&self) -> &str {
        match self {
            ElapsedTime => "",
            PlayPauseIndicator => "",
            TotalTime => "",
        }
    }
}
