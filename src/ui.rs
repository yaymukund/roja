mod cursor;
pub mod handle_ui_events;

pub use crate::ui::cursor::Cursor;
pub use crate::ui::handle_ui_events::handle_ui_events;

use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use std::io::{stdout, Write};

pub struct UI {
    cursor: Cursor,
}

impl UI {
    pub fn new() -> UI {
        UI {
            cursor: Cursor::new(0, 0),
        }
    }

    pub fn draw(&self) {
        self.cursor.draw("hello world");
    }

    pub fn flush(&self) {
        stdout().flush().expect("Could not flush");
    }
}

pub fn init_ui() -> UI {
    enable_raw_mode();
    queue!(stdout(), Clear(ClearType::All)).expect("could not clear screen");
    UI::new()
}

pub fn teardown_ui() {
    disable_raw_mode().expect("could not disable raw mode");
}
