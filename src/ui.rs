use crate::player::HasMetadata;
use crate::player::Player;
use cursive::align::HAlign;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;

use std::thread;
use std::time::Duration;
pub fn create_application() -> Cursive {
    let song_path = "http://localhost:3000/song.mp3";
    let player = Player::new();
    player.play(song_path);
    let mut metadata = player.metadata();

    let mut app = Cursive::default();
    app.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new(song_path))
                .child(TextView::new_with_content(
                    metadata.remove("time-pos").unwrap(),
                )),
        )
        .h_align(HAlign::Center),
    );
    app
}
