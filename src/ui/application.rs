use crate::player::{Player, PlayerProperty};
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView};
use cursive::wrap_impl;

pub struct ApplicationView {
    view: LinearLayout,
}

impl ApplicationView {
    pub fn new(player: &Player) -> ApplicationView {
        let metadata = player.text_contents();
        let elapsed = metadata.get(&PlayerProperty::Elapsed).unwrap().clone();
        let duration = metadata.get(&PlayerProperty::Duration).unwrap().clone();

        let view = LinearLayout::horizontal()
            .child(TextView::new_with_content(elapsed))
            .child(TextView::new("/"))
            .child(TextView::new_with_content(duration));

        ApplicationView { view }
    }
}

impl ViewWrapper for ApplicationView {
    wrap_impl!(self.view: LinearLayout);
}
