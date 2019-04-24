mod runtime_state;
mod views;
use cursive::event::Key;
use cursive::Cursive;
use mpv::Mpv;
pub use runtime_state::Runtime;
pub use views::{ApplicationView, MainView, PlayerView, SearchView};

pub fn create_application<'a>() -> Cursive {
    let mpv = Mpv::new().unwrap();
    let runtime = Runtime::new(&mpv);
    let metadata = runtime.player().get_metadata().clone();
    let song_path = "http://localhost:3000/song.mp3";
    runtime.player().play(song_path);

    let mut app = Cursive::default();
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
