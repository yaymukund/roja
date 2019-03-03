extern crate mpv;

mod player;
use std::env;
use player::{Player, SeekMode};
use std::{thread, time};

fn main() {
    let path = env::args().nth(1).expect("Please provide a URL to an MP3");
    let player = Player::new();
    player.play(&path);
    thread::sleep(time::Duration::from_secs(2));
    player.seek(-15, SeekMode::Absolute);
    player.append("http://localhost:3000/song2.mp3");
    println!("Seeked to last 10s and appended song");
    thread::sleep(time::Duration::from_secs(150));
    let pos = player.elapsed();
    println!("Played {} secs", pos);
}
