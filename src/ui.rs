mod application;
use application::ApplicationView;

use crate::player::Player;
use cursive::Cursive;

pub fn create_application(player: &Player) -> Cursive {
    let mut app = Cursive::default();
    app.set_fps(15);
    app.add_layer(ApplicationView::new(player));
    app
}
