use crate::library::TrackIndex;
use crate::ui::{Event, Listener, State};

impl Listener for TrackIndex {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        if let Event::SelectFolder(folder_id) = event {
            let tracks = self.tracks_for_folder_id(*folder_id);
            ui.dispatch(Event::SetPlaylistTracks(tracks));
        }
    }
}
