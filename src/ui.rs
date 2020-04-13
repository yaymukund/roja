mod component;
mod components;
mod event;
mod helpers;

use crate::runtime::RcRuntime;
pub(crate) use component::{Canvas, UIComponent};
pub(crate) use event::UIEvent;

use crossterm::event::{poll, read, Event as CrosstermEvent};
use crossterm::{cursor, execute, queue, terminal};

use std::convert::TryFrom;
use std::default::Default;
use std::io::{stdout, Write};
use std::time;

pub(crate) struct UI {
    canvas: Canvas,
    runtime: RcRuntime,
    components: Vec<Box<dyn UIComponent>>,
}

impl UI {
    pub(crate) fn new(runtime: RcRuntime) -> UI {
        let (cols, rows) = terminal::size().expect("could not determine size of terminal");
        let mut ui = UI {
            runtime,
            components: Default::default(),
            canvas: Canvas {
                x: 0,
                y: 0,
                cols,
                rows,
            },
        };
        ui.init_components();
        ui
    }

    pub(crate) fn draw(&self) {
        self.emit(UIEvent::Draw);
    }

    pub(crate) fn flush(&self) {
        stdout().flush().expect("Could not flush");
    }

    pub(crate) fn on_external_event<T>(&self, external_event: T)
    where
        UIEvent: TryFrom<T>,
    {
        if let Ok(event) = UIEvent::try_from(external_event) {
            self.emit(event);
        }
    }

    pub(crate) fn poll_crossterm_event(&self) -> Option<CrosstermEvent> {
        let has_event = poll(time::Duration::from_secs(0)).expect("could not poll ui events queue");

        if has_event {
            let event = read().expect("error reading from events queue");
            Some(event)
        } else {
            None
        }
    }

    fn emit(&self, event: UIEvent) {
        for component in self.components.iter() {
            component.on_event(&event, self.runtime.clone());
        }
    }

    fn init_components(&mut self) {
        let keyboard_shortcuts = components::KeyboardShortcuts::new();
        let dashboard = components::Dashboard::new(Canvas {
            x: 1,
            y: self.canvas.rows - 1,
            rows: 1,
            cols: self.canvas.cols - 2,
        });

        self.components.push(Box::new(dashboard));
        self.components.push(Box::new(keyboard_shortcuts))
    }
}

pub(crate) fn init_ui(runtime: RcRuntime) -> UI {
    terminal::enable_raw_mode().expect("could not enable raw mode");
    queue!(
        stdout(),
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )
    .expect("could not hide cursor and clear screen");
    let ui = UI::new(runtime);
    ui.draw();
    ui
}

pub(crate) fn teardown_ui() {
    execute!(
        stdout(),
        cursor::Show,
        terminal::Clear(terminal::ClearType::All),
    )
    .expect("could not display cursor and clear screen");
    terminal::disable_raw_mode().expect("could not disable raw mode");
}
