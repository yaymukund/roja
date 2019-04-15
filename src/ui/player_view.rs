use crate::player::{Metadata, PlayerProperty};
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView};
use cursive::wrap_impl;

pub struct PlayerView {
    view: LinearLayout,
}

impl PlayerView {
    pub fn new(metadata: &Metadata) -> PlayerView {
        let elapsed = metadata.get(&PlayerProperty::Elapsed).unwrap().clone();
        let duration = metadata.get(&PlayerProperty::Duration).unwrap().clone();

        let view = LinearLayout::horizontal()
            .child(TextView::new_with_content(elapsed))
            .child(TextView::new("/"))
            .child(TextView::new_with_content(duration));

        PlayerView { view }
    }
}

impl ViewWrapper for PlayerView {
    wrap_impl!(self.view: LinearLayout);
}
