use crate::ui::Canvas;

pub trait Drawable {
    fn draw(&self, canvas: Canvas);
}
