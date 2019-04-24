use crate::player::Metadata;
use crate::ui::{MainView, PlayerView, SearchView};
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

    pub fn new(metadata: &Metadata) -> ApplicationView {
        let view = LinearLayout::vertical()
            .child(PlayerView::new(metadata))
            .child(BoxView::with_full_screen(MainView::new_with_id()))
            .child(SearchView::new_with_id().full_width());

        ApplicationView { view }
    }

    pub fn new_with_id(metadata: &Metadata) -> IdView<ApplicationView> {
        Self::new(metadata).with_id(Self::ID)
    }

    pub fn focus_id(&mut self, id: impl Into<String>) -> Result<(), ()> {
        self.focus_view(&Selector::Id(&id.into()))
    }
}
