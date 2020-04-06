use std::time;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};

use crate::runtime::RcRuntime;

pub fn handle_ui_events(runtime: RcRuntime) {
    match next_event() {
        Some(Event::Key(key_event)) => handle_key_event(key_event, runtime),
        _ => return,
    }
}

fn handle_key_event(key_event: KeyEvent, runtime: RcRuntime) {
    let KeyEvent { code, modifiers } = key_event;
    match code {
        KeyCode::Left => runtime.borrow().player.seek_backward(),
        KeyCode::Right => runtime.borrow().player.seek_forward(),
        KeyCode::Char('q') => runtime.borrow_mut().stop(),
        _ => println!("unrecognized key event"),
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
