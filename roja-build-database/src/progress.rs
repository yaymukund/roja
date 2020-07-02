use std::io::{stdout, Stdout, Write};

pub struct Progress {
    stdout: Stdout,
    total: usize,
    current: usize,
    added: usize,
    skipped: usize,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self {
            stdout: stdout(),
            total,
            current: 0,
            added: 0,
            skipped: 0,
        }
    }

    pub fn increment_skipped(&mut self) {
        self.skipped += 1;
        self.increment_current();
    }

    pub fn increment_added(&mut self) {
        self.added += 1;
        self.increment_current();
    }

    fn increment_current(&mut self) {
        self.current += 1;
        print!(
            "\r{}/{} completed, {} added ({} skipped)",
            self.current, self.total, self.added, self.skipped
        );
        self.stdout.flush().expect("could not flush stdout");
    }
}
