mod application;
mod main;
mod metadata;
mod player;
mod search;

use crate::runtime::Runtime;
pub use application::ApplicationView;
use cursive::event::Key;
use cursive::Cursive;
pub use main::MainView;
pub use metadata::Metadata;
pub use player::PlayerView;
pub use search::SearchView;

pub fn create_application() -> Cursive {
    let runtime = Runtime::new();
    let metadata = Metadata::new();
    metadata.initialize(&mut runtime.player());
    let song_path = "http://localhost:3000/song.mp3";
    runtime.player().play(song_path);

    let mut app = Cursive::default();
    app.set_user_data(runtime);
    app.set_autorefresh(true);
    app.add_layer(ApplicationView::new_with_id(&metadata));
    setup_global_callbacks(&mut app);
    app
}

pub fn setup_global_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', cb_quit_cursive);
    app.add_global_callback('/', cb_open_search);
    app.add_global_callback(Key::Right, cb_seek_forward);
    app.add_global_callback(Key::Left, cb_seek_backward);
    app.add_global_callback('c', cb_toggle_pause);
}

fn cb_quit_cursive(app: &mut Cursive) {
    app.quit();
}

fn cb_open_search(app: &mut Cursive) {
    app.call_on_id(SearchView::ID, |v: &mut SearchView| {
        v.enable();
        v.clear();
    });
    app.call_on_id(ApplicationView::ID, |v: &mut ApplicationView| {
        v.focus_id(SearchView::ID).unwrap();
    });
}

fn cb_seek_forward(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player().seek_forward();
    });
}

fn cb_seek_backward(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player().seek_backward();
    });
}

fn cb_toggle_pause(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player().toggle_pause();
    });
}
