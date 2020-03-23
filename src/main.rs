mod library;
mod player;
mod runtime;
mod settings;
mod ui;
mod util;

use crate::player::{Player, RcPlayer};
use mpv::Mpv;
use runtime::Runtime;
use ui::create_application;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    env_logger::init();
    let player = init_player();
    let mut app = create_application(player);

    while app.is_running() {
        let runtime: &mut Runtime = app.user_data().unwrap();
        runtime.poll_events();
        app.step();
    }
}

fn init_player() -> RcPlayer {
    let mpv = Mpv::new().unwrap();
    let player = Player::new(mpv);
    Rc::new(RefCell::new(player))
}
