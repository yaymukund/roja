use std::fmt::Display;
use std::io::{stdout, Write};
use std::time;

use crossterm::{cursor, event, event::Event as CrosstermEvent, execute, queue, style, terminal};

pub fn write_at(x: u16, y: u16, text: &str) {
    queue!(stdout(), cursor::MoveTo(x, y), style::Print(text))
        .expect("could not write text to stdout");
}

pub fn write_styled_at<D>(x: u16, y: u16, text: style::StyledContent<D>)
where
    D: Display + Clone,
{
    queue!(
        stdout(),
        cursor::MoveTo(x, y),
        style::PrintStyledContent(text)
    )
    .expect("could not write styled text to stdout");
}

pub fn init() {
    terminal::enable_raw_mode().expect("could not enable raw mode");
    queue!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("could not hide cursor and clear screen");
}

pub fn deinit() {
    execute!(
        stdout(),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All),
    )
    .expect("could not display cursor and clear screen");
    terminal::disable_raw_mode().expect("could not disable raw mode");
}

pub fn poll_event() -> Option<CrosstermEvent> {
    let has_event =
        event::poll(time::Duration::from_secs(0)).expect("could not poll ui events queue");

    if has_event {
        let event = event::read().expect("error reading from events queue");
        Some(event)
    } else {
        None
    }
}
