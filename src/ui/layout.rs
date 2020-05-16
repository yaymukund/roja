use crate::util::Canvas;

#[derive(Debug, Clone)]
pub struct Layout {
    pub folder_view: Canvas,
    pub player: Canvas,
    pub playlist: Canvas,
}

const CENTER_MARGIN: u16 = 1;
const BOTTOM_MARGIN: u16 = 1;
const PLAYER_LEFT_MARGIN: u16 = 1;
const PLAYER_RIGHT_MARGIN: u16 = 1;

impl Layout {
    pub fn new(width: u16, height: u16) -> Self {
        let player = Canvas::new(
            point!(PLAYER_LEFT_MARGIN, height - BOTTOM_MARGIN - 1),
            width - PLAYER_LEFT_MARGIN - PLAYER_RIGHT_MARGIN,
            1,
        );

        let folder_view_width = (width - CENTER_MARGIN) / 3;
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
        assert_eq!(layout.playlist.point(), &point!(4, 0));
        assert_eq!(layout.player.point(), &point!(1, 8));
    }
}
