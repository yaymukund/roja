use cursive::view::{Identifiable, ViewWrapper};
use cursive::views::{DummyView, IdView, LinearLayout};
use cursive::wrap_impl;

pub struct MainView {
    view: LinearLayout,
}

impl ViewWrapper for MainView {
    wrap_impl!(self.view: LinearLayout);
}

impl MainView {
    pub const ID: &'static str = "main";

    pub fn new() -> MainView {
        let view = LinearLayout::horizontal().child(DummyView).child(DummyView);
        MainView { view }
    }

    pub fn new_with_id() -> IdView<MainView> {
        Self::new().with_id(Self::ID)
    }
}
