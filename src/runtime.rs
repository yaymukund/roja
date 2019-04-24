use crate::library::Library;
use crate::player::{Player, PlayerEventHandler};
use crate::settings::Settings;
use mpv::Mpv;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct Runtime {
    pub player: Rc<RefCell<Player<Mpv>>>,
    pub library: Rc<RefCell<Library>>,
}

impl Runtime {
    pub fn new() -> Runtime {
        let mpv = Mpv::new().unwrap();
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());
        let player = Player::new(mpv);

        Runtime {
            player: Rc::new(RefCell::new(player)),
            library: Rc::new(RefCell::new(library)),
        }
    }

    pub fn player(&self) -> Ref<Player<Mpv>> {
        self.player.borrow()
    }

    pub fn poll_events(&self) {
        self.player.borrow_mut().poll_events();
    }
}
