use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::Settings;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Folder {
    id: usize,
    pub created_at: usize,
    pub path: PathBuf,
}

impl Folder {
    pub fn path_str(&self) -> &str {
        self.path.to_str().expect("could not turn path into string")
    }
}

impl Folder {
    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    id: usize,
    title: String,
    album: String,
    artist: String,
    date: String,
    track_number: String,
    duration: String,
    path: PathBuf,
    folder_id: usize,
}

impl Track {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Database {
    tracks: Vec<Track>,
    folders: Vec<Folder>,
}

pub fn read_json() -> (Vec<Folder>, TrackIndex) {
    let path = Settings::global().metadata_path();
    let file = File::open(path).expect("Could not open metadata file");
    let reader = BufReader::new(file);
    let database: Database =
        serde_json::from_reader(reader).expect("Could not read library JSON file");
    let track_index = TrackIndex::new(database.tracks);
    (database.folders, track_index)
}

pub struct TrackIndex {
    tracks: HashMap<usize, Rc<Track>>,
    folder_track_ids: HashMap<usize, Vec<usize>>,
}

impl TrackIndex {
    pub fn new(input_tracks: Vec<Track>) -> Self {
        let mut tracks = HashMap::with_capacity(input_tracks.len());
        let mut folder_track_ids = HashMap::new();

        for track in input_tracks {
            let track_ids = folder_track_ids
                .entry(track.folder_id)
                .or_insert_with(Vec::new);
            track_ids.push(track.id);
            tracks.insert(track.id, Rc::new(track));
        }

        Self {
            tracks,
            folder_track_ids,
        }
    }

    fn get_track(&self, track_id: usize) -> Rc<Track> {
        self.tracks
            .get(&track_id)
            .expect("could not find track by id")
            .clone()
    }

    pub fn tracks_for_folder_id(&self, folder_id: usize) -> Vec<Rc<Track>> {
        self.folder_track_ids
            .get(&folder_id)
            .expect("error: no tracks for folder_id")
            .iter()
            .map(|track_id| self.get_track(*track_id))
            .collect()
    }
}

pub struct Playlist {
    tracks: Vec<Rc<Track>>,
}

impl Playlist {
    pub fn new() -> Self {
        Self { tracks: Vec::new() }
    }

    pub fn tracks(&mut self) -> &mut Vec<Rc<Track>> {
        &mut self.tracks
    }
}
