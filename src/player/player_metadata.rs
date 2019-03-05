// use crate::player::Player;
// use crossbeam_utils::thread;
// use cursive::views::TextContent;
// use mpv::Format;
// use std::collections::HashMap;
//
// pub trait HasMetadata {
//     fn metadata(&self) -> HashMap<String, TextContent>;
// }
//
// impl HasMetadata for Player {
//     fn metadata(&self) -> HashMap<String, TextContent> {
//         self.mpv
//             .observe_property("time-pos", Format::Int64, 0)
//             .unwrap();
//
//         let mut metadata = HashMap::new();
//         let elapsed = TextContent::new("");
//
//         thread::scope(|s| {
//             s.spawn(|_| loop {
//                 let ev = unsafe { self.mpv.wait_event(600.) };
//             });
//         });
//
//         metadata
//     }
// }
