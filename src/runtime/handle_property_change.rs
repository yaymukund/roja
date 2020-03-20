use crate::player::PlayerProperty;
use crate::runtime::Runtime;
use crate::ui::Metadata;
use mpv::events::simple::PropertyData;

pub fn handle_property_change(property: &PlayerProperty, data: &PropertyData, runtime: &Runtime) {
    match property {
        PlayerProperty::Duration | PlayerProperty::Elapsed => {
            copy_property_to_metadata(property, data, runtime)
        }

        PlayerProperty::Pause => {
            update_playing_state(runtime);
        }
    }
}

fn update_playing_state(runtime: &Runtime) {
    let player = runtime.player();

    let indicator = if player.paused() {
        "||"
    } else if player.idle() {
        ""
    } else {
        "►"
    };

    let metadata = runtime.metadata();
    update_property(metadata, &PlayerProperty::PlayState, indicator);
}

fn update_property(metadata: &Metadata, property: &PlayerProperty, text: &str) {
    let mut text_content = metadata.get(property).clone();
    text_content.set_property(text);
}

fn copy_property_to_metadata(property: &PlayerProperty, data: &PropertyData, runtime: &Runtime) {
    let metadata = runtime.metadata();
    let mut text_content = metadata.get(property).clone();
    let text = property.parse_property_data(&data);
    text_content.set_content(&text);
}
