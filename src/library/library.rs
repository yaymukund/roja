use super::{Folder, Store};
use std::path::Path;

pub struct Library {
    store: Store,
    selected_index: usize,
}

impl Library {
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            store: Store::from_path(path),
            selected_index: 0,
        }
    }

    pub fn folders(&self) -> &[Folder] {
        &self.store.folders
    }
}
