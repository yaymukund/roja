use crate::ui::{selectors, Label, LabelSet};
use cursive::view::{View, ViewWrapper};
use cursive::views::{LinearLayout, NamedView, PaddedView, ProgressBar, ResizedView, TextView};
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
        let progress_bar = init_progress_bar();

        let view = LinearLayout::horizontal()
            .child(indicator)
            .child(TextView::new_with_content(elapsed))
            .child(TextView::new("/"))
            .child(TextView::new_with_content(duration))
            .child(progress_bar);

        PlayerView { view }
    }
}

fn init_progress_bar() -> impl View {
    let progress_bar = ProgressBar::new().with_label(|_, _| "".to_string());
    let progress_bar = NamedView::new(selectors::PROGRESS, progress_bar);
    PaddedView::lrtb(1, 0, 0, 0, ResizedView::with_full_width(progress_bar))
}

impl ViewWrapper for PlayerView {
    wrap_impl!(self.view: LinearLayout);
}
