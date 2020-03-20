use crate::ui::{LabelSet, MainView, PlayerView, SearchView};
use cursive::traits::View;
use cursive::view::Boxable;
use cursive::view::{Identifiable, Selector, ViewWrapper};
use cursive::views::{BoxView, IdView, LinearLayout};
use cursive::wrap_impl;

pub struct ApplicationView {
    view: LinearLayout,
}

impl ViewWrapper for ApplicationView {
    wrap_impl!(self.view: LinearLayout);
}

impl ApplicationView {
    pub const ID: &'static str = "application";

    pub fn new(label_set: &LabelSet) -> ApplicationView {
        let view = LinearLayout::vertical()
            .child(PlayerView::new(&label_set))
            .child(BoxView::with_full_screen(MainView::new_with_id()))
            .child(SearchView::new_with_id().full_width());

        ApplicationView { view }
    }

    pub fn new_with_id(label_set: &LabelSet) -> IdView<ApplicationView> {
        Self::new(&label_set).with_id(Self::ID)
    }

    pub fn focus_id(&mut self, id: impl Into<String>) -> Result<(), ()> {
        self.focus_view(&Selector::Id(&id.into()))
    }
}
