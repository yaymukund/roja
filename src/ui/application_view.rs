use crate::player::{MpvApi, Player};
use crate::ui::{PlayerView, SearchView};
use cursive::traits::View;
use cursive::view::Boxable;
use cursive::view::{Identifiable, Selector, ViewWrapper};
use cursive::views::{BoxView, DummyView, IdView, LinearLayout};
use cursive::wrap_impl;

pub struct ApplicationView {
    view: LinearLayout,
}

impl ViewWrapper for ApplicationView {
    wrap_impl!(self.view: LinearLayout);
}

impl ApplicationView {
    pub const ID: &'static str = "application";

    pub fn new<'a, T>(player: &Player<'a, T>) -> ApplicationView
    where
        T: MpvApi,
    {
        let metadata = player.get_metadata();

        let view = LinearLayout::vertical()
            .child(PlayerView::new(metadata))
            .child(BoxView::with_full_screen(DummyView))
            .child(SearchView::new_with_id().full_width());

        ApplicationView { view }
    }

    pub fn new_with_id<'a, T>(player: &Player<'a, T>) -> IdView<ApplicationView>
    where
        T: MpvApi,
    {
        Self::new(player).with_id(Self::ID)
    }

    pub fn focus_id(&mut self, id: impl Into<String>) -> Result<(), ()> {
        self.focus_view(&Selector::Id(&id.into()))
    }
}
