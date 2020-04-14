use crate::runtime::RcRuntime;

use crate::ui::{UIComponent, UIEvent};

pub(crate) struct KeyboardShortcuts {}

impl KeyboardShortcuts {
    pub fn new() -> KeyboardShortcuts {
        KeyboardShortcuts {}
    }
}

impl UIComponent for KeyboardShortcuts {
    fn on_event(&self, event: &UIEvent, runtime: RcRuntime) {
        match *event {
            UIEvent::Quit => runtime.borrow_mut().stop(),
            UIEvent::SeekBackward => runtime.borrow().player.seek_backward(),
            UIEvent::SeekForward => runtime.borrow().player.seek_forward(),
            UIEvent::TogglePause => runtime.borrow().player.toggle_pause(),
            _ => {}
        }
    }

    fn after_event(&self, _event: &UIEvent, _runtime: RcRuntime) {}
}
