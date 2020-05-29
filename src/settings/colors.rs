use crossterm::style::Color;
use serde::Deserialize;

use super::SColor;

#[derive(Deserialize, Debug)]
pub struct Colors {
    highlight: SColor,
    highlight_bg_enabled: SColor,
    highlight_bg_disabled: SColor,
    divider: SColor,
    progress_bar_fill: SColor,
    progress_bar_empty: SColor,
    info_bar: SColor,
    info_bar_bg: SColor,
    controls: SColor,
    controls_bg: SColor,
}

impl Colors {
    pub fn highlight(&self) -> &Color {
        self.highlight.as_ref()
    }
    pub fn highlight_bg_enabled(&self) -> &Color {
        self.highlight_bg_enabled.as_ref()
    }
    pub fn highlight_bg_disabled(&self) -> &Color {
        self.highlight_bg_disabled.as_ref()
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
    pub fn info_bar(&self) -> &Color {
        self.info_bar.as_ref()
    }
    pub fn info_bar_bg(&self) -> &Color {
        self.info_bar_bg.as_ref()
    }
    pub fn controls(&self) -> &Color {
        self.controls.as_ref()
    }
    pub fn controls_bg(&self) -> &Color {
        self.controls_bg.as_ref()
    }
}
