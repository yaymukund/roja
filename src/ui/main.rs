use cursive::view::{Nameable, ViewWrapper};
use cursive::views::{DummyView, LinearLayout, NamedView};
use cursive::wrap_impl;

pub struct MainView {
    view: LinearLayout,
}

impl ViewWrapper for MainView {
    wrap_impl!(self.view: LinearLayout);
}

impl MainView {
    pub const NAME: &'static str = "main";

    pub fn new() -> MainView {
        let view = LinearLayout::horizontal().child(DummyView).child(DummyView);
        MainView { view }
    }

    pub fn new_with_name() -> NamedView<MainView> {
        Self::new().with_name(Self::NAME)
    }
}
