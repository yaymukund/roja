use cursive::view::{Identifiable, ViewWrapper};
use cursive::views::{EditView, IdView};
use cursive::wrap_impl;
use std::ops::{Deref, DerefMut};

pub struct SearchView {
    view: EditView,
}

impl ViewWrapper for SearchView {
    wrap_impl!(self.view: EditView);
}

impl SearchView {
    pub const ID: &'static str = "search";

    pub fn new() -> SearchView {
        let view = EditView::new()
            .on_submit(|c, _| {
                c.call_on_id(Self::ID, |v: &mut SearchView| v.search());
            })
            .content("Enter a search thing")
            .disabled();

        SearchView { view }
    }

    pub fn new_with_id() -> IdView<SearchView> {
        Self::new().with_id(Self::ID)
    }

    pub fn search(&mut self) {
        unimplemented!();
    }
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
