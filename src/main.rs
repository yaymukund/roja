#![warn(rust_2018_idioms)]
#[macro_use]
mod util;
mod library;
mod listener;
mod player;
mod settings;
pub mod store;
mod ui;

use gumdrop::Options;

use std::{thread, time};

pub use settings::{CliOptions, Command, SETTINGS};
use ui::UI;

fn main() {
    env_logger::init();
    player::initialize_mpv();

    let cli_options = CliOptions::parse_args_default_or_exit();
    if let Some(Command::InitDb(opts)) = cli_options.command {
        // init db
        store::initialize_db(&opts.load_path, opts.create);
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
    let event_context = player::create_event_context();

    ui.register(listener::Window);
    ui.register(listener::Focus);
    ui.register(tracks_index);
    ui.register(terminal);
    ui.register(folders);
    ui.register(event_context);
    ui.register(player);
    ui.register(playlist);
    ui.redraw();

    while let Ok(()) = ui.tick() {
        thread::sleep(time::Duration::from_millis(10));
    }
}
