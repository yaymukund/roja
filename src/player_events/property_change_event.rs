use crate::player::PlayerProperty;
use crate::runtime::Runtime;
use crate::ui::{selectors, Label, LabelSet};
use cursive::views::ProgressBar;
use cursive::Cursive;
use mpv::events::simple::PropertyData;

pub fn property_change_event(
    property: &PlayerProperty,
    data: &PropertyData,
    runtime: &Runtime,
    app: &mut Cursive,
) {
    match property {
        PlayerProperty::Duration => {
            let text = property.parse_property_data(&data);
            copy_property_to_label(&Label::TotalTime, text, runtime);
            update_progress_bar(runtime, app);
        }

        PlayerProperty::Elapsed => {
            let text = property.parse_property_data(&data);
            copy_property_to_label(&Label::ElapsedTime, text, runtime);
        }

        PlayerProperty::Pause => {
            update_playing_state(runtime);
        }
    }
}

fn update_progress_bar(runtime: &Runtime, app: &mut Cursive) {
    app.call_on_name(selectors::PROGRESS, |view: &mut ProgressBar| {});
}

fn update_playing_state(runtime: &Runtime) {
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

fn update_property(label_set: &LabelSet, label: &Label, text: String) {
    let text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}

fn copy_property_to_label(label: &Label, text: String, runtime: &Runtime) {
    let label_set = runtime.label_set();
    let text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}
