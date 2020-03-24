use crate::ui::{Label, LABELS};
use cursive::views::TextContent;
use std::collections::HashMap;

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
