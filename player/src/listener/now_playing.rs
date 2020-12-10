use crate::store::Playlist;
use crate::ui::{Event, IntoListener, Listener, Section};
use crate::util::channel;

pub struct NowPlaying;

pub struct NowPlayingListener {
    playlist: Playlist,
    sender: channel::Sender<Event>,
}

impl Listener for NowPlayingListener {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::QueuePlaylist(playlist) => self.playlist = playlist.clone(),
            Event::ChangePlaylistStart(new_index) => {
                self.playlist.selected_index = *new_index as usize;
                let event = Event::DisplayPlaylist(self.playlist.clone());
                self.sender.send(event);
                self.sender.send(Event::Focus(Section::Playlist));
            }
            _ => {}
        }
    }
}

impl IntoListener for NowPlaying {
    type LType = NowPlayingListener;

    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            playlist: Playlist::default(),
            sender: sender.clone(),
        }
    }
}
