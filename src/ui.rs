mod application;
mod label;
mod player;
mod search;
pub mod selectors;
mod track_list;

use crate::runtime::Runtime;
use crate::ui::player::PlayerView;
pub use application::ApplicationView;
use cursive::event::Key;
use cursive::Cursive;
pub use label::{Label, LabelSet, LABELS};
pub use search::SearchView;
pub use track_list::TrackListView;

pub fn create_application(runtime: &Runtime) -> Cursive {
    let label_set = runtime.label_set().clone();
    let mut app = Cursive::default();
    app.set_user_data(runtime.clone());
    app.set_autorefresh(true);
    app.add_layer(ApplicationView::new(&label_set));
    setup_global_callbacks(&mut app);
    app
}

fn setup_global_callbacks(app: &mut Cursive) {
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
    app.call_on_name(SearchView::NAME, |v: &mut SearchView| {
        v.enable();
        v.clear();
    });
    app.call_on_name(ApplicationView::NAME, |v: &mut ApplicationView| {
        v.focus_name(SearchView::NAME).unwrap();
    });
}

fn cb_seek_forward(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player.borrow_mut().seek_forward();
    });
}

fn cb_seek_backward(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player.borrow_mut().seek_backward();
    });
}

fn cb_toggle_pause(app: &mut Cursive) {
    app.with_user_data(|runtime: &mut Runtime| {
        runtime.player.borrow_mut().toggle_pause();
    });
}
