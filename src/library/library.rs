use super::{Folder, Store};
use std::path::Path;

pub struct Library {
    store: Store,
}

impl Library {
    pub fn from_path<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            store: Store::from_path(path),
        }
    }

    pub fn folders(&self) -> &[Folder] {
        &self.store.folders
    }
}
