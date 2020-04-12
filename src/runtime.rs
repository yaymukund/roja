use std::cell::RefCell;
use std::rc::Rc;

use crate::library::Library;
use crate::player::Player;
use crate::settings::Settings;

pub struct Runtime {
    pub stopped: bool,
    pub player: Player,
    pub settings: Settings,
    pub library: Library,
}

pub type RcRuntime = Rc<RefCell<Runtime>>;

impl Runtime {
    pub fn new() -> Runtime {
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());

        Runtime {
            stopped: false,
            player: Player::new(),
            settings,
            library,
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }
}

pub fn init_runtime() -> RcRuntime {
    let runtime = Runtime::new();
    runtime.player.play("http://localhost:3000/song.mp3");
    Rc::new(RefCell::new(runtime))
}
