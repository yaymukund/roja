#![warn(rust_2018_idioms)]
#[macro_use]
mod util;
mod listener;
mod player;
mod settings;
pub mod store;
mod ui;

use std::{thread, time};

use anyhow::Result;
pub use settings::SETTINGS;
use ui::UI;

fn main() -> Result<()> {
    env_logger::init();
    player::initialize_mpv();

    let mut ui = UI::new();

    ui.register(listener::Window);
    ui.register(listener::Focus);
    ui.register(listener::Terminal);
    ui.register(listener::FoldersView);
    ui.register(player::create_event_context());
    ui.register(player::Player::new());
    ui.register(listener::PlaylistView);
    ui.redraw();

    while let Ok(()) = ui.tick() {
        thread::sleep(time::Duration::from_millis(10));
    }

    Ok(())
}
