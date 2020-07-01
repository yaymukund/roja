use std::fmt::Display;

use crossterm::style::{style, Color, Styler};

use crate::util::{terminal, Point};
use crate::SETTINGS;

pub enum Label {
    PlayerControls,
    PlayerInfoBar,
    PlayerProgress,
    PlayerProgressEmpty,
    ListFocusedHighlightedRow,
    ListUnfocusedHighlightedRow,
    ListRow,
    WindowDivider,
}

use Label::*;

impl Label {
    fn bg_color(&self) -> Option<Color> {
        SETTINGS.with(|s| {
            let colors = s.colors();
            match self {
                PlayerControls | PlayerProgress | PlayerProgressEmpty => {
                    Some(*colors.controls_bg())
                }
                PlayerInfoBar => Some(*colors.info_bar_bg()),
                ListFocusedHighlightedRow => Some(*colors.highlight_bg_focused()),
                ListUnfocusedHighlightedRow => Some(*colors.highlight_bg_unfocused()),
                _ => None,
            }
        })
    }

    fn is_bold(&self) -> bool {
        match *self {
            ListFocusedHighlightedRow | ListUnfocusedHighlightedRow => true,
            _ => false,
        }
    }

    fn color(&self) -> Option<Color> {
        SETTINGS.with(|s| {
            let colors = s.colors();
            match self {
                PlayerControls => Some(*colors.controls()),
                PlayerInfoBar => Some(*colors.info_bar()),
                PlayerProgress => Some(*colors.progress_bar_fill()),
                PlayerProgressEmpty => Some(*colors.progress_bar_empty()),
                ListFocusedHighlightedRow | ListUnfocusedHighlightedRow => {
                    Some(*colors.highlight())
                }
                WindowDivider => Some(*colors.divider()),
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
