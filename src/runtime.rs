mod handle_property_change;
use crate::library::Library;
use crate::player::{Evented, Player, PlayerProperty};
use crate::runtime::handle_property_change::handle_property_change;
use crate::settings::Settings;
use crate::ui::Metadata;
use mpv::events::simple::PropertyData;
use mpv::Mpv;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct Runtime {
    pub player: Rc<RefCell<Player<Mpv>>>,
    pub library: Rc<RefCell<Library>>,
    pub metadata: Rc<RefCell<Metadata>>,
}

impl Runtime {
    pub fn new() -> Runtime {
        let mpv = Mpv::new().unwrap();
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());
        let player = Player::new(mpv);
        let metadata = Metadata::new();

        let runtime = Runtime {
            player: Rc::new(RefCell::new(player)),
            library: Rc::new(RefCell::new(library)),
            metadata: Rc::new(RefCell::new(metadata)),
        };

        runtime.listen_for_changes();
        runtime
    }

    pub fn player(&self) -> RefMut<'_, Player<Mpv>> {
        self.player.borrow_mut()
    }

    fn listen_for_changes(&self) {
        let runtime = self.clone();
        let on_property_change = move |property: &PlayerProperty, data: &PropertyData<'_>| {
            handle_property_change(property, data, &runtime);
        };

        self.player()
            .on_property_change(Box::new(on_property_change));
    }

    pub fn poll_events(&self) {
        self.player.borrow_mut().poll_events();
    }

    pub fn metadata(&self) -> Ref<Metadata> {
        self.metadata.borrow()
    }
}
