use crossterm::style::style;

use crate::ui::{Event, IntoListener, Layout, Listener, State};
use crate::util::Canvas;
use crate::Settings;

pub struct Window;

const VLINE: &str = "│";
const HLINE: &str = "─";
const TL_CORNER: &str = "┌";
const TR_CORNER: &str = "┐";

pub struct WindowView {
    canvas: Canvas,
    player_y: u16,
}

impl WindowView {
    fn folder_view_width(&self) -> u16 {
        Layout::folder_view_width(self.canvas.width())
    }

    fn draw(&self) {
        let offset = self.folder_view_width();
        let vline = style(VLINE).with(*Settings::global().colors().divider());

        for y in 0..self.player_y {
            self.canvas
                .right(offset)
                .down(y)
                .write_styled(vline.clone());
        }
    }
}

impl Listener for WindowView {
    fn on_event(&mut self, event: &Event, _ui: &mut State) {
        match event {
            Event::ResizeListener(layout) => {
                self.canvas = layout.window.clone();
                self.player_y = layout.player.point().y();
            }
            Event::Draw => self.draw(),
            _ => {}
        }
    }
}

impl IntoListener for Window {
    type LType = WindowView;

    fn into_listener(self, layout: &Layout) -> Self::LType {
        Self::LType {
            canvas: layout.window.clone(),
            player_y: layout.player.point().y(),
        }
    }
}
