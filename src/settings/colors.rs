use super::SColor;
use crossterm::style::Color;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Colors {
    highlight: SColor,
    highlight_bg: SColor,
    divider: SColor,
    progress_bar_fill: SColor,
    progress_bar_empty: SColor,
}

impl Colors {
    pub fn highlight(&self) -> &Color {
        self.highlight.as_ref()
    }
    pub fn highlight_bg(&self) -> &Color {
        self.highlight_bg.as_ref()
    }
    pub fn divider(&self) -> &Color {
        self.divider.as_ref()
    }
    pub fn progress_bar_fill(&self) -> &Color {
        self.progress_bar_fill.as_ref()
    }
    pub fn progress_bar_empty(&self) -> &Color {
        self.progress_bar_empty.as_ref()
    }
}
