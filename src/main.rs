mod library;
mod player;
mod runtime;
mod settings;
mod ui;
mod util;

use std::{thread, time};

use crate::runtime::init_runtime;
use crate::ui::{handle_ui_events, teardown_ui};

fn main() {
    env_logger::init();
    let runtime = init_runtime();

    loop {
        thread::sleep(time::Duration::from_millis(50));

        if runtime.borrow().stopped {
            break;
        }

        handle_ui_events(runtime.clone());
        runtime.borrow().ui.flush();
    }

    teardown_ui();
}
