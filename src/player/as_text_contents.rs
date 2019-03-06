use crate::player::{Player, PlayerProperty};
use cursive::views::TextContent;
use log::debug;
use std::collections::HashMap;

pub trait AsTextContents {
    fn text_contents(&self) -> HashMap<PlayerProperty, TextContent>;
}

impl<Callback> AsTextContents for Player<Callback>
where
    Callback: FnMut(),
{
    fn text_contents(&self) -> HashMap<PlayerProperty, TextContent> {
        let elapsed = PlayerProperty::Elapsed;
        let text_content = TextContent::new("0");
        let mut text_contents = HashMap::new();

        text_contents.insert(elapsed, text_content.clone());

        self.on_property_change(PlayerProperty::Elapsed, |data| {
            debug!("Changed property to {}", data.to_string());
            text_content.set_content(data.to_string());
        });

        text_contents
    }
}

// thread::scope(|s| {
//     s.spawn(|_| loop {
//         let event = unsafe { self.mpv.wait_event(600.) };
//         if let Some(Ok(Event::PropertyChange {
//             change: PropertyData::Int64(data),
//             ..
//         })) = event
//         {
//             println!("Now the time elapsed is {}", data.to_string());
//             elapsed.set_content(data.to_string());
//         }
//     });
// });
// println!("Finished the crossbean thread");
//
// metadata.insert("elapsed".to_string(), elapsed.clone());
// metadata
