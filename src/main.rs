mod component;
mod library;
mod player;
mod settings;
mod ui;
mod util;

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
    let terminal = component::Terminal::new();

    player.play("http://localhost:3000/song.mp3");

    ui.register(player);
    ui.register(terminal);
    ui
}
