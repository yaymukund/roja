use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent};
use mpv::events::simple::{Event as MpvEvent, PropertyData};

#[derive(PartialEq)]
pub enum Event {
    Draw,

    // Keyboard Input
    Quit,
    SeekBackward,
    SeekForward,
    MoveDown,
    MoveUp,
    TogglePause,

    // MPV Events
    ChangeTotalTime(i64),
    ChangeCurrentTime(i64),
    ChangeIndicator,

    // other
    UnknownMpvEvent,
    UnknownCrosstermEvent,
}

impl<'a> From<MpvEvent<'a>> for Event {
    fn from(mpv_event: MpvEvent) -> Self {
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

            MpvEvent::PropertyChange { name: "pause", .. } => Event::ChangeIndicator,

            _ => Event::UnknownMpvEvent,
        }
    }
}

impl From<CrosstermEvent> for Event {
    fn from(crossterm_event: CrosstermEvent) -> Self {
        if let CrosstermEvent::Key(KeyEvent { code, modifiers: _ }) = crossterm_event {
            match code {
                KeyCode::Left => Event::SeekBackward,
                KeyCode::Right => Event::SeekForward,
                KeyCode::Down => Event::MoveDown,
                KeyCode::Up => Event::MoveUp,
                KeyCode::Char('c') => Event::TogglePause,
                KeyCode::Char('q') => Event::Quit,
                _ => Event::UnknownCrosstermEvent,
            }
        } else {
            Event::UnknownCrosstermEvent
        }
    }
}
