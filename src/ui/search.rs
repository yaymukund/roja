use crate::ui::{ApplicationView, MainView};
use cursive::view::{Nameable, ViewWrapper};
use cursive::views::{EditView, NamedView};
use cursive::wrap_impl;
use cursive::Cursive;
use std::ops::{Deref, DerefMut};

pub struct SearchView {
    view: EditView,
}

impl ViewWrapper for SearchView {
    wrap_impl!(self.view: EditView);
}

impl SearchView {
    pub const NAME: &'static str = "search";

    pub fn new() -> SearchView {
        let view = EditView::new()
            .disabled()
            .on_submit(cb_close_search)
            .on_edit(cb_submit_search);

        SearchView { view }
    }

    pub fn new_with_name() -> NamedView<SearchView> {
        Self::new().with_name(Self::NAME)
    }

    pub fn clear(&mut self) {
        self.set_content("");
    }

    pub fn search(&mut self, _query: &str) {
        // perform search
    }
}

fn cb_close_search(app: &mut Cursive, search_term: &str) {
    cb_submit_search(app, search_term, 0);

    app.call_on_name(SearchView::NAME, |v: &mut SearchView| v.disable());
    app.call_on_name(ApplicationView::NAME, |v: &mut ApplicationView| {
        v.focus_name(MainView::NAME)
    });
}

fn cb_submit_search(app: &mut Cursive, search_term: &str, _cursor_position: usize) {
    app.call_on_name(SearchView::NAME, |v: &mut SearchView| v.search(search_term));
}

impl Deref for SearchView {
    type Target = EditView;

    fn deref(&self) -> &Self::Target {
        &self.view
    }
}

impl DerefMut for SearchView {
    fn deref_mut(&mut self) -> &mut EditView {
        &mut self.view
    }
}
