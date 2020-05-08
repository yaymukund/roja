use super::{List, ListRow, Listable};
use crate::library::{Folder, Library};
use crate::ui::IntoComponent;
use crate::util::Canvas;

impl ListRow for Folder {
    fn row_text(&self) -> &str {
        self.path_str()
    }
}

impl Listable for Library {
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

    fn canvas(cols: u16, rows: u16) -> Canvas {
        Canvas::new(point!(0, 0), cols / 3, rows - 1)
    }
}

impl IntoComponent for Library {
    type IntoComp = List<Library>;

    fn into_component(self, cols: u16, rows: u16) -> Self::IntoComp {
        List::new(self, Library::canvas(cols, rows))
    }
}
