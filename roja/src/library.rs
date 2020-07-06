use crate::store::Track;

pub struct Playlist {
    tracks: Vec<Track>,
}

impl Playlist {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn tracks(&mut self) -> &mut Vec<Track> {
        &mut self.tracks
    }
}
