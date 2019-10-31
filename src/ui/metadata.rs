use crate::player::{Evented, MpvApi, Player, PlayerProperty, PROPERTIES};
use cursive::views::TextContent;
use mpv::events::simple::PropertyData;
use std::collections::HashMap;

pub struct Metadata {
    text_contents: HashMap<PlayerProperty, TextContent>,
}

impl Metadata {
    pub fn new() -> Metadata {
        let mut text_contents: HashMap<PlayerProperty, TextContent> = Default::default();

        for property in PROPERTIES.iter() {
            let text_content = TextContent::new(property.default_value());
            text_contents.insert(property.clone(), text_content);
        }

        Metadata { text_contents }
    }

    pub fn initialize<T>(&self, player: &mut Player<T>)
    where
        T: MpvApi,
    {
        let text_contents = self.text_contents.clone();
        let on_property_change = move |property: &PlayerProperty, data: &PropertyData<'_>| {
            let mut text_contents = text_contents.clone();
            let text_content = text_contents.get_mut(property).unwrap();
            text_content.set_content(property.parse_property_data(&data));
        };

        player.on_property_change(Box::new(on_property_change));
    }

    pub fn get(&self, property: PlayerProperty) -> &TextContent {
        self.text_contents.get(&property).unwrap()
    }
}
