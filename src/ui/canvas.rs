pub(crate) struct Position {
    x: u16,
    y: u16,
}

impl From<(u16, u16)> for Position {
    fn from(pos: (u16, u16)) -> Position {
        Position { x: pos.0, y: pos.1 }
    }
}

pub(crate) struct Canvas {
    pub(crate) x: u16,
    pub(crate) y: u16,
    width: u16,
    height: u16,
}

impl Canvas {
    pub(crate) fn new(x: u16, y: u16, width: u16, height: u16) -> Canvas {
        Canvas {
            x,
            y,
            width,
            height,
        }
    }

    // pub(crate) fn print(&self, text: &str) {
    //     queue!(stdout(), MoveTo(self.x, self.y), Print(text)).unwrap_or_else(|err| {
    //         println!("Error drawing {}", err);
    //     });
    // }

    // pub(crate) fn print_line(&self, text: &str) {
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
