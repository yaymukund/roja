use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use libmpv::events::{Event as MpvEvent, PropertyData};
use libmpv::MpvNode;
use std::collections::HashMap;

use super::Section;
use crate::player::SeekableRanges;
use crate::store::Playlist;

#[derive(Clone, Debug)]
pub enum Event {
    // every tick
    Tick,
    // terminal size changed
    Resize(u16, u16),
    // redraw
    Draw,
    // show playlist
    DisplayPlaylist(Playlist),
    // queue songs in playlist, starting with selected song
    QueuePlaylist(Playlist),
    // switch focus between sections
    Focus(Section),

    // exit
    Quit,
    // pause/unpause
    TogglePause,
    // ← 5s
    SeekBackward,
    // → 5s
    SeekForward,
    // move up/down in the current section
    MoveDown,
    MoveUp,
    PageDown,
    PageUp,
    // change section
    TabFocus,
    // select whatever's highlighted
    Enter,

    // respond to mpv property changes
    ChangeTotalTime(i64),
    ChangeCurrentTime(i64),
    ChangeIndicator,
    ChangeTitle,
    ChangeIdle,
    ChangeSeekableRanges(SeekableRanges),
    ChangePlaylistPos(i64),

    // log & ignore
    UnknownMpvEvent,
    UnknownCrosstermEvent,
}

impl<'a> From<MpvEvent<'a>> for Event {
    fn from(mpv_event: MpvEvent<'_>) -> Self {
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
            } => Event::ChangePlaylistPos(new_index),

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
            CrosstermEvent::Key(KeyEvent { code, modifiers }) => from_key_event(code, modifiers),
            _ => Event::UnknownCrosstermEvent,
        }
    }
}

fn from_key_event(code: KeyCode, modifiers: KeyModifiers) -> Event {
    if modifiers.contains(KeyModifiers::CONTROL) {
        match code {
            KeyCode::Down | KeyCode::Char('j') => return Event::PageDown,
            KeyCode::Up | KeyCode::Char('k') => return Event::PageUp,
            _ => {}
        }
    }

    match code {
        KeyCode::Left | KeyCode::Char('h') => Event::SeekBackward,
        KeyCode::Right | KeyCode::Char('l') => Event::SeekForward,
        KeyCode::Down | KeyCode::Char('j') => Event::MoveDown,
        KeyCode::Up | KeyCode::Char('k') => Event::MoveUp,
        KeyCode::PageUp => Event::PageUp,
        KeyCode::PageDown => Event::PageDown,
        KeyCode::Tab => Event::TabFocus,
        KeyCode::Enter => Event::Enter,
        KeyCode::Char('c') => Event::TogglePause,
        KeyCode::Char('q') => Event::Quit,
        _ => Event::UnknownCrosstermEvent,
    }
}
