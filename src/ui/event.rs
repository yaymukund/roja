use std::convert::TryFrom;

use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use mpv::events::simple::{Event as MpvEvent, PropertyData};

pub(crate) enum UIEvent {
    Draw,

    // Keyboard Input
    Quit,
    SeekBackward,
    SeekForward,
    TogglePause,

    // MPV Events
    ChangeTotalTime(i64),
    ChangeCurrentTime(i64),
    ChangeIndicator,
}

impl<'a> TryFrom<MpvEvent<'a>> for UIEvent {
    type Error = &'static str;

    fn try_from(mpv_event: MpvEvent) -> Result<Self, Self::Error> {
        match mpv_event {
            MpvEvent::PropertyChange {
                name: "time-pos",
                change: PropertyData::Int64(new_time_pos),
                ..
            } => Ok(UIEvent::ChangeCurrentTime(new_time_pos)),

            MpvEvent::PropertyChange {
                name: "duration",
                change: PropertyData::Int64(new_duration),
                ..
            } => Ok(UIEvent::ChangeTotalTime(new_duration)),

            MpvEvent::PropertyChange { name: "pause", .. } => Ok(UIEvent::ChangeIndicator),

            _ => Err("MPV event does not map to UI event"),
        }
    }
}

impl TryFrom<CrosstermEvent> for UIEvent {
    type Error = &'static str;

    fn try_from(crossterm_event: CrosstermEvent) -> Result<Self, Self::Error> {
        if let CrosstermEvent::Key(KeyEvent { code, modifiers: _ }) = crossterm_event {
            match code {
                KeyCode::Left => Ok(UIEvent::SeekBackward),
                KeyCode::Right => Ok(UIEvent::SeekForward),
                KeyCode::Char('c') => Ok(UIEvent::TogglePause),
                KeyCode::Char('q') => Ok(UIEvent::Quit),
                _ => Err("Unrecognized keyboard event"),
            }
        } else {
            Err("Unrecognized Crossterm event")
        }
    }
}
