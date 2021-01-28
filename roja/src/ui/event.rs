use crossterm::event::Event as CrosstermEvent;
pub use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use libmpv::events::{Event as MpvEvent, PropertyData};
use libmpv::MpvNode;
use std::collections::HashMap;
use std::rc::Rc;

use crate::player::SeekableRanges;
use crate::store::Playlist;

#[derive(Clone, Debug)]
pub enum Event {
    Tick,
    // terminal size changed, sent when terminal size changes.
    Resize(u16, u16),
    // redraw, also sent after resize.
    Draw,
    // show playlist in the UI
    DisplayPlaylist(Playlist),
    // queue songs in playlist, replace "now playing" playlist
    QueuePlaylist(Playlist),
    Quit,
    FocusPlaylist,
    FocusFolderList,
    FocusSearch,
    CancelSearch,
    Search(Rc<String>),

    // Keypresses, incl. directional presses.
    Key(KeyEvent),

    // respond to mpv property changes
    ChangeTotalTime(i64),
    ChangeCurrentTime(i64),
    ChangeIndicator,
    ChangeTitle,
    ChangeIdle,
    ChangeSeekableRanges(SeekableRanges),
    ChangePlaylistIndex(i64),

    // log & ignore
    UnknownMpvEvent,
    UnknownCrosstermEvent,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    PageDown,
    PageUp,
}

impl Event {
    pub fn pressed_char(&self) -> Option<char> {
        match self {
            Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) => Some(*c),
            _ => None,
        }
    }

    pub fn is_enter(&self) -> bool {
        matches!(self.keycode(), Some(KeyCode::Enter))
    }

    pub fn keycode(&self) -> Option<&KeyCode> {
        match self {
            Event::Key(KeyEvent { code, .. }) => Some(code),
            _ => None,
        }
    }

    fn key_event(&self) -> Option<KeyEvent> {
        match self {
            Event::Key(key_event) => Some(*key_event),
            _ => None,
        }
    }

    pub fn key_event_direction(&self) -> Option<Direction> {
        let ev = self.key_event()?;

        if ev.modifiers.contains(KeyModifiers::CONTROL) {
            match ev.code {
                KeyCode::Down | KeyCode::Char('j') => return Some(Direction::PageDown),
                KeyCode::Up | KeyCode::Char('k') => return Some(Direction::PageUp),
                _ => {}
            }
        }

        match ev.code {
            KeyCode::PageUp => Some(Direction::PageUp),
            KeyCode::PageDown => Some(Direction::PageDown),
            KeyCode::Char('h') | KeyCode::Left => Some(Direction::Left),
            KeyCode::Char('j') | KeyCode::Down => Some(Direction::Down),
            KeyCode::Char('k') | KeyCode::Up => Some(Direction::Up),
            KeyCode::Char('l') | KeyCode::Right => Some(Direction::Right),
            _ => None,
        }
    }
}

impl<'a> From<MpvEvent<'a>> for Event {
    fn from(mpv_event: MpvEvent<'_>) -> Self {
        log::info!("Found mpv_event: {:?}", mpv_event);
        match mpv_event {
            MpvEvent::PropertyChange {
                name: "time-pos",
                change: PropertyData::Int64(new_time_pos),
                ..
            } => Event::ChangeCurrentTime(new_time_pos),

            MpvEvent::PropertyChange {
                name: "duration",
                change: PropertyData::Int64(new_duration),
                ..
            } => Event::ChangeTotalTime(new_duration),

            MpvEvent::PropertyChange {
                name: "media-title",
                ..
            } => Event::ChangeTitle,

            MpvEvent::PropertyChange {
                name: "core-idle", ..
            } => Event::ChangeIdle,

            MpvEvent::PropertyChange { name: "pause", .. } => Event::ChangeIndicator,

            MpvEvent::PropertyChange {
                name: "demuxer-cache-state",
                change: PropertyData::Node(mpv_node),
                ..
            } => {
                let data = seekable_ranges(mpv_node).expect("could not parse demuxer-cache-state");
                Event::ChangeSeekableRanges(data)
            }

            MpvEvent::PropertyChange {
                name: "playlist-pos",
                change: PropertyData::Int64(new_index),
                ..
            } if new_index >= 0 => Event::ChangePlaylistIndex(new_index),

            _ => Event::UnknownMpvEvent,
        }
    }
}

fn seekable_ranges(demuxer_cache_state: &MpvNode) -> Option<SeekableRanges> {
    let mut res = Vec::new();
    let props: HashMap<&str, MpvNode> = demuxer_cache_state.to_map()?.collect();
    let ranges = props.get("seekable-ranges")?.to_array()?;

    for node in ranges {
        let range: HashMap<&str, MpvNode> = node.to_map()?.collect();
        let start = range.get("start")?.to_f64()?;
        let end = range.get("end")?.to_f64()?;
        res.push((start, end));
    }

    Some(res)
}

impl From<CrosstermEvent> for Event {
    fn from(crossterm_event: CrosstermEvent) -> Self {
        match crossterm_event {
            CrosstermEvent::Resize(cols, rows) => Event::Resize(cols, rows),
            CrosstermEvent::Key(key_event) => Event::Key(key_event),
            _ => Event::UnknownCrosstermEvent,
        }
    }
}
