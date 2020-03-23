mod event;
mod event_handler;
mod handle_property_change;
use crate::library::Library;
use crate::player::{Player, PlayerProperty, RcPlayer};
use crate::runtime::event::Event;
use crate::runtime::event_handler::EventHandler;
use crate::runtime::handle_property_change::handle_property_change;
use crate::settings::Settings;
use crate::ui::LabelSet;
use mpv::events::simple::PropertyData;
use mpv::Mpv;
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub struct Runtime {
    pub player: RcPlayer,
    pub library: Rc<RefCell<Library>>,
    pub label_set: Rc<RefCell<LabelSet>>,
    event_handler: Rc<RefCell<EventHandler>>,
}

impl Runtime {
    pub fn new(player: RcPlayer) -> Runtime {
        let settings = Settings::new();
        let library = Library::from_path(settings.metadata_path());
        let label_set = LabelSet::new();

        let mut runtime = Runtime {
            player,
            library: Rc::new(RefCell::new(library)),
            label_set: Rc::new(RefCell::new(label_set)),
            event_handler: Default::default(),
        };

        runtime.listen_for_changes();
        runtime
    }

    pub fn label_set(&self) -> Ref<LabelSet> {
        self.label_set.borrow()
    }

    pub fn player(&self) -> RefMut<'_, Player<Mpv>> {
        self.player.borrow_mut()
    }

    fn listen_for_changes(&mut self) {
        let runtime = self.clone();
        let on_property_change = move |property: &PlayerProperty, data: &PropertyData<'_>| {
            handle_property_change(property, data, &runtime);
        };

        self.event_handler
            .borrow_mut()
            .on_property_change(Box::new(on_property_change));
    }

    pub fn poll_events(&self) {
        if let Some(event) = self.player.borrow().poll_events() {
            let event = Event::from(event);
            self.event_handler.borrow().trigger(event);
        }
    }
}
