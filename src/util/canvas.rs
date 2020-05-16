use std::fmt::Display;

use super::terminal;

#[macro_export]
macro_rules! point {
    ( $x:expr , $y:expr ) => {{
        crate::util::Point::new($x, $y)
    }};
}

#[derive(PartialEq, Debug, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> u16 {
        self.x
    }

    pub fn y(&self) -> u16 {
        self.y
    }

    pub fn right(&self, offset: u16) -> Self {
        Self::new(self.x + offset, self.y)
    }

    pub fn down(&self, offset: u16) -> Self {
        Self::new(self.x, self.y + offset)
    }

    #[allow(dead_code)]
    pub fn left(&self, offset: u16) -> Self {
        Self::new(self.x - offset, self.y)
    }

    #[allow(dead_code)]
    pub fn up(&self, offset: u16) -> Self {
        Self::new(self.x, self.y - offset)
    }

    pub fn write(&self, text: &str) -> &Self {
        terminal::write_at(self, text);
        self
    }

    pub fn write_styled<D>(&self, text: terminal::StyledContent<D>) -> &Self
    where
        D: Display + Clone,
    {
        terminal::write_styled_at(self, text);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Canvas {
    point: Point,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(point: Point, width: u16, height: u16) -> Self {
        Self {
            point,
            width,
            height,
        }
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn point(&self) -> &Point {
        &self.point
    }

    pub fn right(&self, offset: u16) -> Point {
        self.point.right(offset)
    }

    #[allow(dead_code)]
    pub fn down(&self, offset: u16) -> Point {
        self.point.down(offset)
    }

    #[allow(dead_code)]
    pub fn left(&self, offset: u16) -> Point {
        self.point.left(offset)
    }

    #[allow(dead_code)]
    pub fn up(&self, offset: u16) -> Point {
        self.point.up(offset)
    }

    #[allow(dead_code)]
    pub fn write(&self, text: &str) -> &Self {
        self.point.write(text);
        self
    }

    #[allow(dead_code)]
    pub fn write_styled<D>(&self, text: terminal::StyledContent<D>) -> &Self
    where
        D: Display + Clone,
    {
        self.point.write_styled(text);
        self
    }
}
