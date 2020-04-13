mod library;
mod player;
mod runtime;
mod settings;
mod ui;
mod util;

use std::{thread, time};

use crate::runtime::init_runtime;
use crate::ui::{init_ui, teardown_ui};

fn main() {
    env_logger::init();
    let runtime = init_runtime();
    let ui = init_ui(runtime.clone());

    loop {
        thread::sleep(time::Duration::from_millis(50));

        if runtime.borrow().stopped {
            break;
        }

        if let Some(event) = runtime.borrow().player.poll_event() {
            ui.on_external_event(event);
        }

        if let Some(event) = ui.poll_crossterm_event() {
            ui.on_external_event(event);
        }

        ui.flush();
    }

    teardown_ui();
}
