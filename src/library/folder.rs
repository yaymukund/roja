use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Folder {
    id: usize,
    created_at: usize,
    path: PathBuf,
}
