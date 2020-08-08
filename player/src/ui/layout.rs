use crate::util::Canvas;

const CENTER_MARGIN: u16 = 1;
const BOTTOM_MARGIN: u16 = 1;
const PLAYER_LEFT_MARGIN: u16 = 0;
const PLAYER_RIGHT_MARGIN: u16 = 0;
const PLAYER_HEIGHT: u16 = 2;

pub fn folders_view_width(width: u16) -> u16 {
    (width - CENTER_MARGIN) / 3
}

fn main_height(height: u16) -> u16 {
    height - BOTTOM_MARGIN - PLAYER_HEIGHT
}

pub fn player_y(height: u16) -> u16 {
    height - BOTTOM_MARGIN - 2
}

pub fn window_canvas(width: u16, height: u16) -> Canvas {
    Canvas::new(point!(0, 0), width, height)
}

pub fn player_canvas(width: u16, height: u16) -> Canvas {
    let y = player_y(height);
    Canvas::new(
        point!(PLAYER_LEFT_MARGIN, y),
        width - PLAYER_LEFT_MARGIN - PLAYER_RIGHT_MARGIN,
        PLAYER_HEIGHT,
    )
}

pub fn folders_view_canvas(width: u16, height: u16) -> Canvas {
    let width = folders_view_width(width);
    let height = main_height(height);
    Canvas::new(point!(0, 0), width, height)
}

pub fn playlist_canvas(width: u16, height: u16) -> Canvas {
    let fwidth = folders_view_width(width);
    let x = fwidth + CENTER_MARGIN;
    let height = main_height(height);
    Canvas::new(point!(x, 0), width - fwidth - CENTER_MARGIN, height)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_folders_view_canvas() {
        let canvas = folders_view_canvas(10, 10);
        assert_eq!(canvas.point(), &point!(0, 0));
        assert_eq!(canvas.width(), 3);
        assert_eq!(canvas.height(), 7);
    }

    #[test]
    fn test_playlist_canvas() {
        let canvas = playlist_canvas(10, 10);
        assert_eq!(canvas.point(), &point!(4, 0));
        assert_eq!(canvas.width(), 6);
        assert_eq!(canvas.height(), 7);
    }

    #[test]
    fn test_player_canvas() {
        let canvas = player_canvas(10, 10);
        assert_eq!(canvas.point(), &point!(0, 7));
        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 2);
    }

    #[test]
    fn test_window_canvas() {
        let canvas = window_canvas(10, 10);
        assert_eq!(canvas.point(), &point!(0, 0));
        assert_eq!(canvas.width(), 10);
        assert_eq!(canvas.height(), 10);
    }
}
