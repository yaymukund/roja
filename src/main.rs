pub mod cursive_table_view;
mod library;
mod player;
mod player_events;
mod runtime;
mod settings;
mod ui;
mod util;

use crate::player::{Player, RcPlayer};
use mpv::Mpv;
use player_events::handle_player_event;
use runtime::Runtime;
use ui::Roja;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    env_logger::init();
    let player = init_player();
    let runtime = init_runtime(&player);
    let folders = runtime.library.borrow().folders.clone();
    let mut roja = Roja::new(&runtime, folders);

    start_player(&player);

    while roja.is_running() {
        if let Some(event) = player.borrow().poll_events() {
            handle_player_event(event, &runtime, &mut roja);
        }

        roja.step();
    }
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
