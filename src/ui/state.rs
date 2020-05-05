use crossterm::terminal;

pub struct State {
    stopped: bool,
    rows: u16,
    cols: u16,
}

impl Default for State {
    fn default() -> Self {
        let (cols, rows) = terminal::size().expect("Could not determine size of terminal");
        State {
            stopped: false,
            rows,
            cols,
        }
    }
}

impl State {
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn stopped(&self) -> bool {
        self.stopped
    }

    pub fn rows(&self) -> u16 {
        self.rows
    }

    pub fn cols(&self) -> u16 {
        self.cols
    }
}
