use crate::ui::label::{Label, LABELS};
use cursive::views::TextContent;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Metadata {
    text_contents: HashMap<Label, TextContent>,
}

impl Metadata {
    pub fn new() -> Metadata {
        let mut text_contents: HashMap<Label, TextContent> = Default::default();

        for label in LABELS.iter() {
            let text_content = TextContent::new(label.default_value());
            text_contents.insert(label.clone(), text_content);
        }

        Metadata { text_contents }
    }

    pub fn get(&self, label: &Label) -> &TextContent {
        self.text_contents.get(&label).unwrap()
    }
}
