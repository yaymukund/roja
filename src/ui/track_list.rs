use cursive::view::{Nameable, ViewWrapper};
use cursive::views::{DummyView, LinearLayout, NamedView};
use cursive::wrap_impl;

pub struct TrackListView {
    view: LinearLayout,
}

impl ViewWrapper for TrackListView {
    wrap_impl!(self.view: LinearLayout);
}

impl TrackListView {
    pub const NAME: &'static str = "main";

    pub fn new() -> NamedView<TrackListView> {
        let view = LinearLayout::horizontal();
        let track_list_view = TrackListView { view };
        track_list_view.with_name(Self::NAME)
    }
}
