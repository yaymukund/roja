use std::io::{stdout, Write};

use crossterm::{cursor, queue, style, terminal};

use crate::player::Player;
use crate::ui::Canvas;
use crate::ui::Drawable;

impl Drawable for Player {
    fn draw(&self, canvas: Canvas) {
        let player = format!("{}", indicator(self));

        queue!(
            stdout(),
            cursor::MoveTo(canvas.x, canvas.y),
            style::Print(player),
            terminal::Clear(terminal::ClearType::UntilNewLine),
        )
        .expect("could not draw player");
    }
}

fn indicator(player: &Player) -> &str {
    if player.paused() {
        "|"
    } else if player.core_idle() {
        " "
    } else {
        "▶"
    }
}
