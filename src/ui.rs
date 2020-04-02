mod application;
mod label;
mod player;
mod search;
pub mod selectors;
mod track_list;

use crate::library::Folder;
use crate::runtime::Runtime;
use crate::ui::player::PlayerView;
pub use application::ApplicationView;
use cursive::event::Key;
use cursive::views::ProgressBar;
use cursive::Cursive;
pub use label::{Label, LabelSet, LABELS};
pub use search::SearchView;
pub use track_list::{FolderTable, TrackListView};

pub struct Roja {
    siv: Cursive,
}

impl Roja {
    pub fn new(runtime: &Runtime, folders: Vec<Folder>) -> Roja {
        let mut siv = Cursive::default();
        siv.set_user_data(runtime.clone());
        siv.set_autorefresh(true);

        let label_set = runtime.label_set().clone();
        let application_view = ApplicationView::new(&label_set);
        siv.add_layer(application_view);

        siv.add_global_callback('q', Self::cb_quit);
        siv.add_global_callback('/', Self::cb_open_search);
        siv.add_global_callback(Key::Right, Self::cb_seek_forward);
        siv.add_global_callback(Key::Left, Self::cb_seek_backward);
        siv.add_global_callback('c', Self::cb_toggle_pause);

        let mut roja = Roja { siv };
        roja.set_folders(folders);

        roja
    }

    fn set_folders(&mut self, folders: Vec<Folder>) {
        self.siv
            .call_on_name(TrackListView::NAME, |v: &mut FolderTable| {
                v.set_items(folders);
            });
    }

    fn cb_quit(siv: &mut Cursive) {
        siv.quit();
    }

    fn cb_open_search(siv: &mut Cursive) {
        siv.call_on_name(SearchView::NAME, |v: &mut SearchView| {
            v.enable();
            v.clear();
        });
        siv.call_on_name(ApplicationView::NAME, |v: &mut ApplicationView| {
            v.focus_name(SearchView::NAME).unwrap();
        });
    }

    fn cb_seek_forward(siv: &mut Cursive) {
        siv.with_user_data(|runtime: &mut Runtime| {
            runtime.player.borrow_mut().seek_forward();
        });
    }

    fn cb_seek_backward(siv: &mut Cursive) {
        siv.with_user_data(|runtime: &mut Runtime| {
            runtime.player.borrow_mut().seek_backward();
        });
    }

    fn cb_toggle_pause(siv: &mut Cursive) {
        siv.with_user_data(|runtime: &mut Runtime| {
            runtime.player.borrow_mut().toggle_pause();
        });
    }

    pub fn is_running(&self) -> bool {
        self.siv.is_running()
    }

    pub fn step(&mut self) {
        self.siv.step();
    }

    pub fn update_progress_value(&mut self, percent: usize) {
        self.siv
            .call_on_name(selectors::PROGRESS, |view: &mut ProgressBar| {
                view.set_value(percent);
            });
    }
}
