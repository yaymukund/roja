use std::fmt::Display;

use crossterm::style::{style, Color, Styler};

use crate::util::{terminal, Point};
use crate::SETTINGS;

pub enum Label {
    Console,
    ConsoleHighlighted,
    PlayerControls,
    PlayerInfoBar,
    PlayerProgress,
    PlayerProgressEmpty,
    PlayerProgressBuffered,
    ListFocusedHighlightedRow,
    ListUnfocusedHighlightedRow,
    ListRow,
    ListTitle,
    WindowDivider,
}

use Label::*;

impl Label {
    fn bg_color(&self) -> Option<Color> {
        SETTINGS.with(|s| {
            let colors = s.colors();
            match self {
                PlayerControls | PlayerProgress | PlayerProgressEmpty | PlayerProgressBuffered => {
                    Some(*colors.controls_bg())
                }
                ConsoleHighlighted | PlayerInfoBar | ListTitle | ListFocusedHighlightedRow => {
                    Some(*colors.main())
                }
                ListUnfocusedHighlightedRow => Some(*colors.unfocused()),
                _ => None,
            }
        })
    }

    fn is_bold(&self) -> bool {
        match *self {
            PlayerProgress
            | ListFocusedHighlightedRow
            | ListTitle
            | ListUnfocusedHighlightedRow => true,
            _ => false,
        }
    }

    fn color(&self) -> Option<Color> {
        SETTINGS.with(|s| {
            let colors = s.colors();
            match self {
                WindowDivider => Some(*colors.main()),
                ListFocusedHighlightedRow | ListTitle | PlayerInfoBar => Some(*colors.text_bold()),
                PlayerProgress => Some(*colors.accent()),
                PlayerProgressBuffered => Some(*colors.accent2()),
                PlayerControls => Some(*colors.text_dark()),
                _ => None,
            }
        })
    }

    fn is_styled(&self) -> bool {
        self.is_bold() || self.color().is_some() || self.bg_color().is_some()
    }

    fn style<D>(&self, text: D) -> terminal::StyledContent<D>
    where
        D: Display + Clone,
    {
        let mut text = style(text);

        if self.is_bold() {
            text = text.bold();
        }

        if let Some(color) = self.color() {
            text = text.with(color);
        }

        if let Some(color) = self.bg_color() {
            text = text.on(color);
        }

        text
    }

    pub fn draw_at<T>(&self, point: &Point, text: T)
    where
        T: AsRef<str> + Display + Clone,
    {
        if self.is_styled() {
            terminal::draw_styled_at(point, self.style(text));
        } else {
            terminal::draw_at(point, text);
        }
    }
}
