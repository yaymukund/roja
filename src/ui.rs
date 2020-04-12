mod canvas;
mod drawable;
mod player;

use crate::runtime::RcRuntime;
pub use crate::ui::canvas::Canvas;
pub use crate::ui::drawable::Drawable;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use crossterm::{cursor, execute, queue, terminal};
use mpv::events::simple::Event as MpvEvent;

use std::io::{stdout, Write};
use std::time;

pub struct UI {
    cols: u16,
    rows: u16,
    runtime: RcRuntime,
}

impl UI {
    pub fn new(runtime: RcRuntime) -> UI {
        let (cols, rows) = terminal::size().expect("could not determine size of terminal");
        UI {
            cols,
            rows,
            runtime,
        }
    }

    pub fn draw(&self) {
        let runtime = self.runtime.borrow();
        let canvas = Canvas::new(1, 1, self.cols - 1, 1);
        runtime.player.draw(canvas);
    }

    pub fn flush(&self) {
        stdout().flush().expect("Could not flush");
    }

    pub fn on_ui_event(&self, event: Event) {
        if let Event::Key(KeyEvent { code, modifiers: _ }) = event {
            match code {
                KeyCode::Left => self.runtime.borrow().player.seek_backward(),
                KeyCode::Right => self.runtime.borrow().player.seek_forward(),
                KeyCode::Char('c') => self.runtime.borrow().player.toggle_pause(),
                KeyCode::Char('q') => self.runtime.borrow_mut().stop(),
                _ => return,
            }
        }
    }

    pub fn on_player_event(&self, event: MpvEvent) {}
}

pub fn init_ui(runtime: RcRuntime) -> UI {
    terminal::enable_raw_mode().expect("could not enable raw mode");
    queue!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("could not hide cursor and clear screen");
    UI::new(runtime.clone())
}

pub fn teardown_ui() {
    execute!(
        stdout(),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All),
    )
    .expect("could not display cursor and clear screen");
    terminal::disable_raw_mode().expect("could not disable raw mode");
}

pub fn poll_crossterm_event() -> Option<Event> {
    let has_event = poll(time::Duration::from_secs(0)).expect("could not poll ui events queue");

    if has_event {
        let event = read().expect("error reading from events queue");
        Some(event)
    } else {
        None
    }
}
