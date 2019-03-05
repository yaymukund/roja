use crate::player::{Player, PlayerProperty};
use cursive::align::HAlign;
use cursive::views::{Dialog, LinearLayout, TextView};
use cursive::Cursive;
use std::{thread, time};

pub fn create_application() -> Cursive {
    let song_path = "http://localhost:3000/song.mp3";
    let mut player = Player::new();
    player.play(song_path);
    thread::sleep(time::Duration::from_millis(2000));
    player.poll_events();

    let metadata = player.text_contents();
    let elapsed = metadata.get(&PlayerProperty::Elapsed).unwrap().clone();
    let mut app = Cursive::default();

    app.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(TextView::new(song_path))
                .child(TextView::new_with_content(elapsed)),
        )
        .h_align(HAlign::Center),
    );
    app
}
