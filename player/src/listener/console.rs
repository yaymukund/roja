use crate::store::Playlist;
use crate::ui::{layout, Event, IntoListener, Label, Listener, Section};
use crate::util::{channel, terminal, Canvas};

const SEARCH_PREFIX: &str = "/";

pub struct Console;

pub struct ConsoleListener {
    canvas: Canvas,
    search_text: String,
    enabled: bool,
    sender: channel::Sender<Event>,
}

impl ConsoleListener {
    fn resize(&mut self, width: u16, height: u16) {
        self.canvas = layout::console_canvas(width, height);
    }

    fn enable(&mut self) {
        self.enabled = true;
        self.draw();
    }

    fn draw(&mut self) {
        if !self.enabled {
            return;
        }

        let width = self.canvas.width() as usize;
        self.canvas.draw(
            format!(
                "{}{:width$}",
                SEARCH_PREFIX,
                self.search_text,
                width = width - SEARCH_PREFIX.len()
            ),
            Label::Console,
        );
    }
}

impl Listener for ConsoleListener {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::Focus(Section::Search) => self.enable(),
            Event::Resize(width, height) => self.resize(*width, *height),
            _ => {}
        }
    }
}

impl IntoListener for Console {
    type LType = ConsoleListener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            canvas: Canvas::Uninitialized,
            enabled: false,
            search_text: "".to_string(),
            sender: sender.clone(),
        }
    }
}
