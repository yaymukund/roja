use cursive::views::TextContent;
use std::collections::HashMap;

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

#[derive(Clone)]
pub struct LabelSet {
    text_contents: HashMap<Label, TextContent>,
}

impl LabelSet {
    pub fn new() -> LabelSet {
        let mut text_contents: HashMap<Label, TextContent> = Default::default();

        for label in LABELS.iter() {
            let text_content = TextContent::new(label.default_value());
            text_contents.insert(label.clone(), text_content);
        }

        LabelSet { text_contents }
    }

    pub fn get(&self, label: &Label) -> &TextContent {
        self.text_contents.get(&label).unwrap()
    }
}

impl Default for LabelSet {
    fn default() -> LabelSet {
        LabelSet::new()
    }
}
