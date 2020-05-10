use crossterm::event::{Event as CrosstermEvent, KeyCode, KeyEvent, KeyModifiers};
use mpv::events::simple::{Event as MpvEvent, PropertyData};

#[derive(PartialEq)]
pub enum Event {
    Tick,
    Resize(u16, u16),
    Draw,

    // Keyboard Input
    Quit,
    SeekBackward,
    SeekForward,
    MoveDown,
    MoveUp,
    PageDown,
    PageUp,
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

            MpvEvent::PropertyChange { name: "pause", .. } => Event::ChangeIndicator,

            _ => Event::UnknownMpvEvent,
        }
    }
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
        KeyCode::Char('c') => Event::TogglePause,
        KeyCode::Char('q') => Event::Quit,
        _ => Event::UnknownCrosstermEvent,
    }
}
