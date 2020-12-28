// use crate::store::Playlist;
use crate::ui::{layout, Event, IntoListener, KeyCode, KeyEvent, Label, Listener, Section};
use crate::util::{channel, Canvas};

const SEARCH_PREFIX: &str = "/";
const LEFT_OFFSET: u16 = SEARCH_PREFIX.len() as u16;

pub struct Console;

pub struct ConsoleListener {
    canvas: Canvas,
    text: String,
    text_start_idx: u16,
    cursor_offset: u16,
    searching: bool,
    sender: channel::Sender<Event>,
}

impl ConsoleListener {
    fn resize(&mut self, width: u16, height: u16) {
        self.canvas = layout::console_canvas(width, height);
        self.draw();
    }

    fn enable(&mut self) {
        self.searching = true;
        self.draw();
    }

    fn width(&self) -> u16 {
        self.canvas.width() - LEFT_OFFSET
    }

    fn cursor_idx(&self) -> u16 {
        self.text_start_idx + self.cursor_offset
    }

    fn draw(&mut self) {
        if !self.searching {
            return;
        }

        // render the prefix ("/")
        self.canvas.draw(SEARCH_PREFIX, Label::Console);
        let mut cursor = self.canvas.right(LEFT_OFFSET);

        // Used to convert the char to a &str. See:
        //
        // https://stackoverflow.com/a/47634755
        let mut tmp = [0; 4];
        let mut chars = self.text.chars().skip(self.text_start_idx as usize);

        for i in 0..self.width() {
            let c = chars.next().unwrap_or(' ');
            let label = if i == self.cursor_offset {
                Label::ConsoleHighlighted
            } else {
                Label::Console
            };

            let c = (c as char).encode_utf8(&mut tmp);
            cursor.draw(&c, label);
            cursor = cursor.right(1);
        }
    }

    fn on_key_char(&mut self, c: &char) {
        self.text.insert(self.cursor_idx() as usize, *c);
        let end = self.width() - 1;

        if self.cursor_offset == end {
            self.text_start_idx += 1;
        } else {
            self.cursor_offset += 1;
        }

        self.draw();
    }

    fn on_key_backspace(&mut self) {
        let cursor_idx = self.cursor_idx();

        if cursor_idx == 0 {
            return;
        }

        if self.cursor_offset == 0 {
            self.on_key_left();
        } else {
            self.text.remove(cursor_idx as usize - 1);
            self.cursor_offset -= 1;
            self.draw();
        }
    }

    fn on_key_left(&mut self) {
        match (self.text_start_idx, self.cursor_offset) {
            (0, 0) => return,
            (_, 0) => {
                let amount = std::cmp::min(self.text_start_idx, self.width());
                self.text_start_idx -= amount;
                self.cursor_offset += amount;
            }
            _ => self.cursor_offset -= 1,
        }

        self.draw();
    }

    fn on_key_right(&mut self) {
        match (self.text_start_idx, self.cursor_offset) {
            (idx, cursor_idx) if (idx + cursor_idx) >= self.text.len() as u16 => return,
            (_, cursor_idx) if cursor_idx == (self.width() - 1) => {
                self.text_start_idx = self.text_start_idx + self.width() - 1;
                self.cursor_offset = 0;
            }
            _ => self.cursor_offset += 1,
        }

        self.draw();
    }
}

impl Listener for ConsoleListener {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::Draw => self.draw(),
            Event::Focus(Section::Search) => self.enable(),
            Event::Resize(width, height) => self.resize(*width, *height),
            Event::Key(KeyEvent { code, .. }) if self.searching => match code {
                KeyCode::Char(c) => self.on_key_char(c),
                KeyCode::Backspace => self.on_key_backspace(),
                KeyCode::Left => self.on_key_left(),
                KeyCode::Right => self.on_key_right(),
                _ => {}
            },
            _ => {}
        }
    }
}

impl IntoListener for Console {
    type LType = ConsoleListener;
    fn into_listener(self, sender: channel::Sender<Event>) -> Self::LType {
        Self::LType {
            canvas: Canvas::Uninitialized,
            cursor_offset: 0,
            text_start_idx: 0,
            searching: false,
            text: "".to_string(),
            sender: sender.clone(),
        }
    }
}
