mod player;
mod ui;

use env_logger::{Builder, Target};
use log::debug;
use ui::create_application;

fn main() {
    let mut log_builder = Builder::new();
    log_builder.target(Target::Stderr);
    log_builder.init();

    let (mut player, mut app) = create_application();
    loop {
        player.poll_events();
        app.step();
    }
}
