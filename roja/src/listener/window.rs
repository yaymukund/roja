use crate::ui::{layout, Event, IntoListener, Label, Listener};
use crate::util::{channel, Canvas};

pub struct Window;

const VLINE: &str = "│";

pub struct WindowView {
    canvas: Canvas,
    player_y: u16,
}

impl WindowView {
    fn folder_view_width(&self) -> u16 {
        layout::folders_view_width(self.canvas.width())
    }

    fn draw(&self) {
        let offset = self.folder_view_width();

        self.canvas.right(offset).draw(" ", Label::ListTitle);

        for y in 1..self.player_y {
            self.canvas
                .right(offset)
                .down(y)
                .draw(VLINE, Label::WindowDivider);
        }
    }
}

impl Listener for WindowView {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::Resize(width, height) => {
                self.canvas = layout::window_canvas(*width, *height);
                self.player_y = layout::player_y(*height);
            }
            Event::Draw => self.draw(),
            _ => {}
        }
    }
}

impl IntoListener for Window {
    type LType = WindowView;

    fn into_listener(self, _sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            canvas: Canvas::Uninitialized,
            player_y: 0,
        }
    }
}
