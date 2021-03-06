#![warn(rust_2018_idioms)]
#[macro_use]
mod util;
mod listener;
mod player;
mod settings;
pub mod store;
mod ui;

use crate::ui::Loop;
use anyhow::Result;
use std::{thread, time};

const TICK_INTERVAL_MS: time::Duration = time::Duration::from_millis(10);

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
    ui.register(listener::NowPlaying);
    ui.register(listener::Search);
    ui.register(listener::Console);
    ui.redraw()?;

    while let Loop::Continue = ui.tick()? {
        thread::sleep(TICK_INTERVAL_MS);
    }

    ui.tick()?;
    Ok(())
}
