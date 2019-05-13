use crate::player::PlayerProperty;
use crate::ui::Metadata;
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView};
use cursive::wrap_impl;

pub struct PlayerView {
    view: LinearLayout,
}

impl PlayerView {
    pub fn new(metadata: &Metadata) -> PlayerView {
        let elapsed = metadata.get(PlayerProperty::Elapsed).clone();
        let duration = metadata.get(PlayerProperty::Duration).clone();

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
