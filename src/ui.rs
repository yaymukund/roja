mod application_view;
mod player_view;
mod search_view;
use crate::player::{MpvApi, Player};
pub use application_view::ApplicationView;
use cursive::Cursive;
pub use player_view::PlayerView;
pub use search_view::SearchView;

pub fn create_application<T>(player: &Player<T>) -> Cursive
where
    T: MpvApi,
{
    let mut app = Cursive::default();
    app.set_autorefresh(true);
    app.add_layer(ApplicationView::new_with_id(player));
    setup_global_callbacks(&mut app);
    app
}

pub fn setup_global_callbacks(app: &mut Cursive) {
    app.add_global_callback('q', quit_cursive);
    app.add_global_callback('/', toggle_search);
}

fn quit_cursive(app: &mut Cursive) {
    app.quit();
}

fn toggle_search(app: &mut Cursive) {
    app.call_on_id(ApplicationView::ID, |view: &mut ApplicationView| {
        view.focus_id(SearchView::ID).unwrap();
    });
}
