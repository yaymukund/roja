pub struct Position {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Position {
    fn from(pos: (u16, u16)) -> Position {
        Position { x: pos.0, y: pos.1 }
    }
}

pub struct Canvas {
    pub x: u16,
    pub y: u16,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Canvas {
        Canvas {
            x,
            y,
            width,
            height,
        }
    }

    // pub fn print(&self, text: &str) {
    //     queue!(stdout(), MoveTo(self.x, self.y), Print(text)).unwrap_or_else(|err| {
    //         println!("Error drawing {}", err);
    //     });
    // }

    // pub fn print_line(&self, text: &str) {
    //     queue!(
    //         stdout(),
    //         MoveTo(self.pos.x, self.pos.y),
    //         Print(text),
    //         Clear(ClearType::UntilNewLine)
    //     )
    //     .unwrap_or_else(|err| {
    //         println!("Error drawing {}", err);
    //     });
    // }

    // fn draw_styled(&self, text: &str, foreground: Color, background: Color) {
    //     queue!(
    //         stdout(),
    //         MoveTo(self.x, self.y),
    //         SetForegroundColor(foreground),
    //         SetBackgroundColor(background),
    //         Print(text),
    //         ResetColor,
    //     )
    //     .unwrap_or_else(|err| {
    //         println!("Error drawing styled {}", err);
    //     });
    // }
}
