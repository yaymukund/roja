use std::fmt::Display;
use std::io::{stdout, Write};

use crossterm::{cursor, queue, style};

pub(crate) fn write_at(x: u16, y: u16, text: &str) {
    queue!(stdout(), cursor::MoveTo(x, y), style::Print(text))
        .expect("could not write text to stdout");
}

pub(crate) fn write_styled_at<D>(x: u16, y: u16, text: style::StyledContent<D>)
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
