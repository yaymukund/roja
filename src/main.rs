mod player;
mod ui;

use ui::create_application;

fn main() {
    let mut app = create_application();
    app.run();

    // use player::{Player, SeekMode};
    // use std::{thread, time};
    // let player = Player::new();
    // player.play("http://localhost:3000/song.mp3");
    // thread::sleep(time::Duration::from_secs(2));
    // player.seek(-15, SeekMode::Absolute);
    // player.append("http://localhost:3000/song2.mp3");
    // println!("Seeked to last 10s and appended song");
    // thread::sleep(time::Duration::from_secs(150));
    // let pos = player.elapsed();
    // println!("Played {} secs", pos);
}
