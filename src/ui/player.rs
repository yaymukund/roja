use crate::ui::{Label, LabelSet};
use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, PaddedView, TextView};
use cursive::wrap_impl;

pub struct PlayerView {
    view: LinearLayout,
}

impl PlayerView {
    pub fn new(label_set: &LabelSet) -> PlayerView {
        let elapsed = label_set.get(&Label::ElapsedTime).clone();
        let duration = label_set.get(&Label::TotalTime).clone();
        let play_pause = label_set.get(&Label::PlayPauseIndicator).clone();

        let indicator = PaddedView::lrtb(0, 1, 0, 0, TextView::new_with_content(play_pause));

        let view = LinearLayout::horizontal()
            .child(indicator)
            .child(TextView::new_with_content(elapsed))
            .child(TextView::new("/"))
            .child(TextView::new_with_content(duration));

        PlayerView { view }
    }
}

impl ViewWrapper for PlayerView {
    wrap_impl!(self.view: LinearLayout);
}
