use std::time;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::runtime::RcRuntime;

pub fn handle_ui_events(runtime: RcRuntime) {
    if let Some(Event::Key(KeyEvent { code, modifiers })) = next_event() {
        match code {
            KeyCode::Left => runtime.borrow().player.seek_backward(),
            KeyCode::Right => runtime.borrow().player.seek_forward(),
            KeyCode::Char('q') => runtime.borrow_mut().stop(),
            _ => return,
        }
    }
}

fn next_event() -> Option<Event> {
    let has_event = poll(time::Duration::from_secs(0)).expect("could not poll ui events queue");

    if has_event {
        let event = read().expect("error reading from events queue");
        Some(event)
    } else {
        None
    }
}
