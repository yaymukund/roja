use std::cmp::Ordering;

use cursive::view::{Nameable, ViewWrapper};
use cursive::views::NamedView;
use cursive::wrap_impl;

use crate::cursive_table_view::{TableView, TableViewItem};
use crate::library::Folder;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum FolderColumn {
    Path,
}

impl TableViewItem<FolderColumn> for Folder {
    fn to_column(&self, _: FolderColumn) -> String {
        self.path.to_str().unwrap().to_string()
    }

    fn cmp(&self, other: &Self, _: FolderColumn) -> Ordering
    where
        Self: Sized,
    {
        self.created_at.cmp(&other.created_at)
    }
}

pub type FolderTable = TableView<Folder, FolderColumn>;

pub struct TrackListView {
    view: NamedView<FolderTable>,
}

impl ViewWrapper for TrackListView {
    wrap_impl!(self.view: NamedView<FolderTable>);
}

impl TrackListView {
    pub const NAME: &'static str = "track_list";

    pub fn new() -> TrackListView {
        let mut view = FolderTable::new().column(FolderColumn::Path, "Folder", |c| c.width(20));

        view.sort_by(FolderColumn::Path, Ordering::Greater);
        view.disable_events();

        TrackListView {
            view: view.with_name(Self::NAME),
        }
    }
}
