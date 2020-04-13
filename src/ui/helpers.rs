use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

pub(crate) fn write_at(x: u16, y: u16, text: &str) {
    queue!(stdout(), cursor::MoveTo(x, y), style::Print(text))
        .expect("could not write text to stdout");
}
