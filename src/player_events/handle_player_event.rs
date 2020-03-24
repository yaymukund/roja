use crate::runtime::Runtime;
use crate::ui::{selectors, Label, LabelSet};
use crate::util::format_duration;
use cursive::views::ProgressBar;
use cursive::Cursive;
use mpv::events::simple::{Event as MpvEvent, PropertyData};

pub fn handle_player_event(event: MpvEvent, runtime: &Runtime, app: &mut Cursive) {
    match event {
        MpvEvent::PropertyChange {
            name: "time-pos",
            change: PropertyData::Int64(new_time_pos),
            ..
        } => handle_time_pos_event(new_time_pos, &runtime, app),

        MpvEvent::PropertyChange {
            name: "duration",
            change: PropertyData::Int64(new_duration),
            ..
        } => handle_duration_event(new_duration, &runtime),

        MpvEvent::PropertyChange { name: "pause", .. } => handle_pause_event(&runtime),

        _ => (),
    }
}

fn handle_time_pos_event(new_time_pos: i64, runtime: &Runtime, app: &mut Cursive) {
    let elapsed_time = format_duration(new_time_pos);
    copy_property_to_label(&Label::ElapsedTime, elapsed_time, runtime);
    update_progress_value(runtime, app);
}

fn handle_duration_event(new_duration: i64, runtime: &Runtime) {
    let total_time = format_duration(new_duration);
    copy_property_to_label(&Label::TotalTime, total_time, runtime);
}

fn handle_pause_event(runtime: &Runtime) {
    let player = runtime.player.borrow();

    let indicator = if player.paused() {
        "▋▋"
    } else if player.core_idle() {
        ""
    } else {
        "►"
    };

    let label_set = runtime.label_set();
    update_property(
        &label_set,
        &Label::PlayPauseIndicator,
        indicator.to_string(),
    );
}

fn update_progress_value(runtime: &Runtime, app: &mut Cursive) {
    let player = runtime.player.borrow();
    let percent = player.percent_complete();

    app.call_on_name(selectors::PROGRESS, |view: &mut ProgressBar| {
        view.set_value(percent);
    });
}

fn update_property(label_set: &LabelSet, label: &Label, text: String) {
    let text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}

fn copy_property_to_label(label: &Label, text: String, runtime: &Runtime) {
    let label_set = runtime.label_set();
    let text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}
