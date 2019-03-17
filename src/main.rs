mod player;
mod settings;
mod ui;
mod util;

use env_logger::{Builder, Target};
use mpv::Mpv;
use player::{Player, PlayerEventHandler as _};
use settings::Settings;
use ui::create_application;

fn main() {
    init_logging();

    let settings = Settings::new();

    let song_path = "http://localhost:3000/song.mp3";
    let mpv = Mpv::new().unwrap();
    let mut player = Player::new(&mpv);
    let mut app = create_application(&player);

    player.play(song_path);

    loop {
        player.poll_events();
        app.step();
    }
}

fn init_logging() {
    let mut log_builder = Builder::new();
    log_builder.target(Target::Stderr);
    log_builder.init();
}
