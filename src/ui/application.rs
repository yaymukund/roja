use crate::ui::{LabelSet, MainView, PlayerView, SearchView};
use cursive::traits::View;
use cursive::view::{Nameable, Resizable, Selector, ViewWrapper};
use cursive::views::{LinearLayout, NamedView, ResizedView};
use cursive::wrap_impl;

pub struct ApplicationView {
    view: LinearLayout,
}

impl ViewWrapper for ApplicationView {
    wrap_impl!(self.view: LinearLayout);
}

impl ApplicationView {
    pub const NAME: &'static str = "application";

    pub fn new(label_set: &LabelSet) -> ApplicationView {
        let view = LinearLayout::vertical()
            .child(PlayerView::new(&label_set))
            .child(ResizedView::with_full_screen(MainView::new_with_name()))
            .child(SearchView::new_with_name().full_width());

        ApplicationView { view }
    }

    pub fn new_with_name(label_set: &LabelSet) -> NamedView<ApplicationView> {
        Self::new(&label_set).with_name(Self::NAME)
    }

    pub fn focus_name(&mut self, name: impl Into<String>) -> Result<(), ()> {
        self.focus_view(&Selector::Name(&name.into()))
    }
}
