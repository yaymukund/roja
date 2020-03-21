use crate::player::PlayerProperty;
use crate::runtime::Runtime;
use crate::ui::{Label, LabelSet};
use mpv::events::simple::PropertyData;

pub fn handle_property_change(property: &PlayerProperty, data: &PropertyData, runtime: &Runtime) {
    match property {
        PlayerProperty::Duration => {
            let text = property.parse_property_data(&data);
            copy_property_to_label(&Label::TotalTime, text, runtime);
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

fn update_playing_state(runtime: &Runtime) {
    let player = runtime.player.borrow();

    let indicator = if player.paused() {
        "||"
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
    let mut text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}

fn copy_property_to_label(label: &Label, text: String, runtime: &Runtime) {
    let label_set = runtime.label_set();
    let mut text_content = label_set.get(label).clone();
    text_content.set_content(&text);
}
