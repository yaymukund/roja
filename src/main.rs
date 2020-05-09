#[macro_use]
mod util;

mod component;
mod library;
mod player;
mod settings;
mod ui;

use std::{thread, time};

use ui::UI;

fn main() {
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

    let terminal = component::Terminal::new();

    let settings = settings::Settings::new();
    let library = library::Library::from_path(settings.metadata_path());

    let player = player::Player::new();
    player.play("http://localhost:3000/song.mp3");

    ui.register(terminal);
    ui.register(library);
    ui.register(player);
    ui
}
