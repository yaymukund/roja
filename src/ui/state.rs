use crossterm::terminal;

pub struct State {
    stopped: bool,
    rows: u16,
    cols: u16,
    library_selected_index: usize,
}

impl Default for State {
    fn default() -> Self {
        let (cols, rows) = terminal::size().expect("Could not determine size of terminal");
        State {
            stopped: false,
            rows,
            cols,
            library_selected_index: 0,
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

    pub fn library_selected_index(&self) -> usize {
        self.library_selected_index
    }

    pub fn library_select(&mut self, new_index: usize) {
        self.library_selected_index = new_index;
    }
}
