use crate::store::Playlist;
use crate::ui::{Event, IntoListener, Listener};
use crate::util::{channel, SendDiscard};
use anyhow::Result;

pub struct NowPlaying;

pub struct NowPlayingListener {
    playlist: Playlist,
    sender: channel::Sender<Event>,
}

impl NowPlayingListener {
    fn display_now_playing(&mut self) -> Result<()> {
        self.sender
            .send_discard(Event::DisplayPlaylist(self.playlist.clone()))
    }
}

impl Listener for NowPlayingListener {
    fn on_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::QueuePlaylist(playlist) => self.playlist = playlist.clone(),
            Event::CancelSearch => self.display_now_playing()?,
            Event::ChangePlaylistIndex(new_index) => {
                self.playlist.selected_index = *new_index as usize;
                self.display_now_playing()?;
            }
            _ => {}
        }

        Ok(())
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
