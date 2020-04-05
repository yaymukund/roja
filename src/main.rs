mod library;
mod player;
mod runtime;
mod settings;
mod util;

use crate::player::{Player, RcPlayer};
use mpv::Mpv;
use runtime::Runtime;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    env_logger::init();
    let player = init_player();
    let runtime = init_runtime(&player);
    let folders = runtime.library.borrow().folders.clone();
    start_player(&player);
    loop {}
}

fn init_player() -> RcPlayer {
    let mpv = Mpv::new().unwrap();
    let player = Player::new(mpv);
    Rc::new(RefCell::new(player))
}

fn init_runtime(player: &RcPlayer) -> Runtime {
    let player = player.clone();
    Runtime::new(player)
}

fn start_player(player: &RcPlayer) {
    player.borrow().play("http://localhost:3000/song.mp3");
}
