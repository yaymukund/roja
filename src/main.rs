#![warn(rust_2018_idioms)]
#[macro_use]
mod util;

mod library;
mod listener;
mod player;
mod settings;
mod ui;

use std::{thread, time};

use ui::UI;

fn main() {
    env_logger::init();
    let mut ui = init_ui();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        ui.tick();

        if ui.stopped() {
            break;
        }
    }
}

fn init_ui() -> UI {
    let mut ui = UI::new();

    let terminal = listener::Terminal::new();

    let settings = settings::Settings::new();
    let (folders, tracks_index) = library::read_json(settings.metadata_path());

    let playlist = library::Playlist::new();

    let player = player::Player::new();
    player.play("http://localhost:3000/song.mp3");

    ui.register(tracks_index);
    ui.register(terminal);
    ui.register(folders);
    ui.register(player);
    ui.register(playlist);
    ui
}
