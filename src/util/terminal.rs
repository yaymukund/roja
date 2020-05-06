use super::Point;
use std::fmt::Display;
use std::io::{stdout, Write};
use std::time;

pub use crossterm::style::StyledContent;
use crossterm::{cursor, event, event::Event as CrosstermEvent, queue, style, terminal};

pub fn size() -> (u16, u16) {
    terminal::size().expect("could not determine size of terminal")
}

pub fn write_at(point: &Point, text: &str) {
    queue!(
        stdout(),
        cursor::MoveTo(point.x(), point.y()),
        style::Print(text)
    )
    .expect("could not write text to stdout");
}

pub fn write_styled_at<D>(point: &Point, text: StyledContent<D>)
where
    D: Display + Clone,
{
    queue!(
        stdout(),
        cursor::MoveTo(point.x(), point.y()),
        style::PrintStyledContent(text)
    )
    .expect("could not write styled text to stdout");
}

pub fn init() {
    terminal::enable_raw_mode().expect("could not enable raw mode");
    clear_all();
    queue!(stdout(), cursor::Hide,).expect("could not hide cursor and clear screen");
}

pub fn flush() {
    stdout().flush().expect("could not flush terminal");
}

pub fn clear_all() {
    queue!(stdout(), terminal::Clear(terminal::ClearType::All),).expect("could not clear terminal");
}

pub fn deinit() {
    queue!(stdout(), cursor::Show,).expect("could not display cursor and clear screen");
    clear_all();
    flush();
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
