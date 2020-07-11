use crossterm::style::Color;
use serde::Deserialize;

use super::SColor;

#[derive(Deserialize, Debug)]
pub struct Colors {
    main: SColor,
    unfocused: SColor,
    controls_bg: SColor,
    accent: SColor,
    accent2: SColor,
    text_bold: SColor,
    text_dark: SColor,
}

impl Colors {
    pub fn main(&self) -> &Color {
        self.main.as_ref()
    }

    pub fn unfocused(&self) -> &Color {
        self.unfocused.as_ref()
    }

    pub fn controls_bg(&self) -> &Color {
        self.controls_bg.as_ref()
    }

    pub fn accent(&self) -> &Color {
        self.accent.as_ref()
    }

    pub fn accent2(&self) -> &Color {
        self.accent2.as_ref()
    }

    pub fn text_bold(&self) -> &Color {
        self.text_bold.as_ref()
    }

    pub fn text_dark(&self) -> &Color {
        self.text_dark.as_ref()
    }
}
