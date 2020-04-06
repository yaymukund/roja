use std::cell::RefCell;
use std::rc::Rc;

use mpv::Mpv;

use crate::library::Library;
use crate::player::Player;
use crate::settings::Settings;
use crate::ui::{init_ui, UI};

pub struct Runtime {
    pub stopped: bool,
    pub player: Player<Mpv>,
    pub settings: Settings,
    pub library: Library,
    pub ui: UI,
}

pub type RcRuntime = Rc<RefCell<Runtime>>;

impl Runtime {
    pub fn new() -> Runtime {
        let mpv = Mpv::new().expect("could not initialize mpv instance");
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());

        Runtime {
            library,
            player: Player::new(mpv),
            settings,
            stopped: false,
            ui: init_ui(),
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
