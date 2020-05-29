#![warn(rust_2018_idioms)]
#[macro_use]
mod util;

mod library;
mod listener;
mod player;
mod settings;
mod ui;

use std::{thread, time};

pub use settings::Settings;
use ui::UI;

fn main() {
    env_logger::init();
    Settings::init();

    let mut ui = UI::new();
    let terminal = listener::Terminal;
    let (folders, tracks_index) = library::read_json();
    let playlist = library::Playlist::new();
    let player = player::Player::new();

    ui.register(listener::Window);
    ui.register(listener::Focus);
    ui.register(tracks_index);
    ui.register(terminal);
    ui.register(folders);
    ui.register(player);
    ui.register(playlist);
    ui.first_draw();

    while let Ok(()) = ui.tick() {
        thread::sleep(time::Duration::from_millis(10));
    }
}
