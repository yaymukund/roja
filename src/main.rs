#![warn(rust_2018_idioms)]
#[macro_use]
mod util;
mod library;
mod listener;
mod player;
mod settings;
mod store;
mod ui;

use gumdrop::Options;

use std::{thread, time};

pub use settings::{CliOptions, Command, SETTINGS};
pub use store::CONNECTION;
use ui::UI;

fn main() {
    env_logger::init();

    let cli_options = CliOptions::parse_args_default_or_exit();
    if let Some(Command::InitDb(_)) = cli_options.command {
        // init db
    } else {
        start_player();
    }
}

fn start_player() {
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
    ui.redraw();

    while let Ok(()) = ui.tick() {
        thread::sleep(time::Duration::from_millis(10));
    }
}
