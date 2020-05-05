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
    env_logger::init();

    let mut ui = init_ui();
    ui.draw();

    loop {
        thread::sleep(time::Duration::from_millis(10));
        ui.tick();

        if ui.stopped() {
            break;
        }
    }
}

fn init_ui() -> UI {
    let mut ui = UI::default();

    let player = player::Player::new();
    player.play("http://localhost:3000/song.mp3");

    let terminal = component::Terminal::new();

    let settings = settings::Settings::new();
    let library = library::Library::from_path(settings.metadata_path());
    let library_list = component::List::new(
        library,
        point!(0, 0),
        ui.state().cols() / 3,
        ui.state().rows() - 1,
    );

    ui.register(player);
    ui.register(terminal);
    ui.register(library_list);
    ui
}
