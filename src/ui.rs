mod application;
use application::ApplicationView;

use crate::player::{MpvApi, Player};
use cursive::Cursive;

pub fn create_application<T>(player: &Player<T>) -> Cursive
where
    T: MpvApi,
{
    let mut app = Cursive::default();
    app.set_fps(15);
    app.add_layer(ApplicationView::new(player));
    app
}
