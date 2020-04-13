use std::cell::RefCell;
use std::rc::Rc;

use crate::library::Library;
use crate::player::Player;
use crate::settings::Settings;

pub(crate) struct Runtime {
    pub(crate) stopped: bool,
    pub(crate) player: Player,
    pub(crate) settings: Settings,
    pub(crate) library: Library,
}

pub(crate) type RcRuntime = Rc<RefCell<Runtime>>;

impl Runtime {
    pub(crate) fn new() -> Runtime {
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());

        Runtime {
            stopped: false,
            player: Player::new(),
            settings,
            library,
        }
    }

    pub(crate) fn stop(&mut self) {
        self.stopped = true;
    }
}

pub(crate) fn init_runtime() -> RcRuntime {
    let runtime = Runtime::new();
    runtime.player.play("http://localhost:3000/song.mp3");
    Rc::new(RefCell::new(runtime))
}
