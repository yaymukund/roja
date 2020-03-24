use crate::library::Library;
use crate::player::RcPlayer;
use crate::settings::Settings;
use crate::ui::LabelSet;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct Runtime {
    pub player: RcPlayer,
    pub library: Rc<RefCell<Library>>,
    pub label_set: Rc<RefCell<LabelSet>>,
}

impl Runtime {
    pub fn new(player: RcPlayer) -> Runtime {
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());

        Runtime {
            player,
            library: Rc::new(RefCell::new(library)),
            label_set: Default::default(),
        }
    }

    pub fn label_set(&self) -> Ref<LabelSet> {
        self.label_set.borrow()
    }
}
