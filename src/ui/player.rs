use crate::player::PlayerProperty;
use crate::ui::metadata::Metadata;
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView};
use cursive::wrap_impl;

pub struct PlayerView {
    view: LinearLayout,
}

impl PlayerView {
    pub fn new(metadata: &Metadata) -> PlayerView {
        let elapsed = metadata.get(&PlayerProperty::Elapsed).clone();
        let duration = metadata.get(&PlayerProperty::Duration).clone();
        let indicator = metadata.get(&PlayerProperty::Indicator).clone();

        let view = LinearLayout::horizontal()
            .child(TextView::new_with_content(indicator))
            .child(TextView::new_with_content(elapsed))
            .child(TextView::new("/"))
            .child(TextView::new_with_content(duration));

        PlayerView { view }
    }
}

impl ViewWrapper for PlayerView {
    wrap_impl!(self.view: LinearLayout);
}
