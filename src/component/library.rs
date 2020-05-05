use super::{ListRow, ListStore};
use crate::library::{Folder, Library};

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

impl ListStore for Library {
    type RowItem = Folder;

    fn get(&self, index: usize) -> &Folder {
        &self.folders()[index]
    }

    fn count(&self) -> usize {
        self.folders().len()
    }

    fn on_select(&mut self) {
        // TODO
    }
}
