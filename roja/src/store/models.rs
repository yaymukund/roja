use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Folder {
    pub id: usize,
    pub created_at: usize,
    pub path: PathBuf,
}

impl Folder {
    pub fn path_str(&self) -> &str {
        self.path.to_str().expect("could not turn path into string")
    }
}

#[derive(Clone, Debug)]
pub struct Track {
    pub id: usize,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub date: String,
    pub track_number: String,
    pub duration: String,
    pub path: PathBuf,
    pub folder_id: usize,
}
