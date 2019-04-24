mod library;
mod player;
mod runtime;
mod settings;
mod ui;
mod util;

use runtime::Runtime;
use ui::create_application;

fn main() {
    let mut app = create_application();

    while app.is_running() {
        let runtime: &mut Runtime = app.user_data().unwrap();
        runtime.poll_events();
        app.step();
    }
}
