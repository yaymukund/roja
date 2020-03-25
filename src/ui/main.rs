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

    pub fn new() -> NamedView<MainView> {
        let view = LinearLayout::horizontal();
        let main_view = MainView { view };
        main_view.with_name(Self::NAME)
    }
}
