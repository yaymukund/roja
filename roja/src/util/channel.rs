use crate::ui::Event;
use anyhow::{anyhow, Result};
pub use crossbeam_channel::*;

pub trait SendDiscard {
    fn send_discard(&self, event: Event) -> Result<()>;
}

impl SendDiscard for Sender<Event> {
    fn send_discard(&self, event: Event) -> Result<()> {
        self.send(event)
            .map_err(|e| anyhow!("Error sending on channel: {}", e))
    }
}
