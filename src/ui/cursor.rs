use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};

pub struct Cursor {
    x: u16,
    y: u16,
}

impl Cursor {
    pub fn new(x: u16, y: u16) -> Cursor {
        Cursor { x, y }
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }

    pub fn draw(&self, text: &str) {
        queue!(stdout(), MoveTo(self.x, self.y), Print(text)).unwrap_or_else(|err| {
            println!("Error drawing {}", err);
        });
    }

    fn draw_styled(&self, text: &str, foreground: Color, background: Color) {
        queue!(
            stdout(),
            MoveTo(self.x, self.y),
            SetForegroundColor(foreground),
            SetBackgroundColor(background),
            Print(text),
            ResetColor,
        )
        .unwrap_or_else(|err| {
            println!("Error drawing styled {}", err);
        });
    }
}
