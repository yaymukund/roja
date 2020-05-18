use crossterm::style::Color;

use crate::util::Canvas;

#[derive(Debug, Clone)]
pub struct Colors {
    pub highlight: Color,
    pub highlight_bg: Color,
    pub divider: Color,
    pub progress_bar_fill: Color,
    pub progress_bar_empty: Color,
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub folder_view: Canvas,
    pub player: Canvas,
    pub playlist: Canvas,
    pub window: Canvas,
    pub colors: Colors,
}

const CENTER_MARGIN: u16 = 1;
const BOTTOM_MARGIN: u16 = 1;
const PLAYER_LEFT_MARGIN: u16 = 1;
const PLAYER_RIGHT_MARGIN: u16 = 1;

impl Layout {
    pub fn folder_view_width(width: u16) -> u16 {
        (width - CENTER_MARGIN) / 3
    }

    pub fn new(width: u16, height: u16) -> Self {
        let window = Canvas::new(point!(0, 0), width, height);
        let player = Canvas::new(
            point!(PLAYER_LEFT_MARGIN, height - BOTTOM_MARGIN - 1),
            width - PLAYER_LEFT_MARGIN - PLAYER_RIGHT_MARGIN,
            1,
        );

        let folder_view_width = Self::folder_view_width(width);
        let playlist_start_x = folder_view_width + CENTER_MARGIN;
        let main_height = height - BOTTOM_MARGIN - player.height();

        let folder_view = Canvas::new(point!(0, 0), folder_view_width, main_height);

        let playlist = Canvas::new(
            point!(playlist_start_x, 0),
            width - folder_view.width() - CENTER_MARGIN,
            main_height,
        );

        Self {
            folder_view,
            player,
            playlist,
            window,
            colors: Colors {
                highlight: Color::White,
                highlight_bg: Color::Magenta,
                divider: Color::Rgb {
                    r: 57,
                    g: 59,
                    b: 96,
                },
                progress_bar_fill: Color::Magenta,
                progress_bar_empty: Color::Green,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_layout() {
        let layout = Layout::new(10, 10);
        assert_eq!(layout.folder_view.point(), &point!(0, 0));
        assert_eq!(layout.folder_view.width(), 3);
        assert_eq!(layout.folder_view.height(), 8);

        assert_eq!(layout.playlist.point(), &point!(4, 0));
        assert_eq!(layout.playlist.width(), 6);
        assert_eq!(layout.playlist.height(), 8);

        assert_eq!(layout.player.point(), &point!(1, 8));
        assert_eq!(layout.player.width(), 8);
        assert_eq!(layout.player.height(), 1);

        assert_eq!(layout.window.point(), &point!(0, 0));
        assert_eq!(layout.window.width(), 10);
        assert_eq!(layout.window.height(), 10);
    }
}
