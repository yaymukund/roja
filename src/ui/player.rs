use crate::player::PlayerProperty;
use crate::ui::{Label, LabelSet};
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, TextView};
use cursive::wrap_impl;

pub struct PlayerView {
    view: LinearLayout,
}

impl PlayerView {
    pub fn new(label_set: &LabelSet) -> PlayerView {
        let track_time = label_set.get(&Label::TrackTime).clone();
        let play_pause = label_set.get(&Label::PlayPauseIndicator).clone();

        let view = LinearLayout::horizontal()
            .child(TextView::new_with_content(play_pause))
            .child(TextView::new_with_content(track_time));

        PlayerView { view }
    }
}

impl ViewWrapper for PlayerView {
    wrap_impl!(self.view: LinearLayout);
}
