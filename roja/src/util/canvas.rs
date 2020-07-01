use std::fmt::Display;

use crate::ui::Label;

pub enum Canvas {
    Uninitialized,
    Initialized(Area),
}

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

    pub fn draw<T>(&self, text: T, label: Label) -> &Self
    where
        T: AsRef<str> + Clone + Display,
    {
        label.draw_at(self, text);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Area {
    point: Point,
    width: u16,
    height: u16,
}

impl Canvas {
    pub fn new(point: Point, width: u16, height: u16) -> Self {
        Self::Initialized(Area {
            point,
            width,
            height,
        })
    }

    fn area(&self) -> &Area {
        match self {
            Self::Uninitialized => panic!("canvas not initialized yet"),
            Self::Initialized(area) => &area,
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self {
            Self::Uninitialized => false,
            _ => true,
        }
    }

    pub fn width(&self) -> u16 {
        self.area().width
    }

    pub fn height(&self) -> u16 {
        self.area().height
    }

    pub fn point(&self) -> &Point {
        &self.area().point
    }

    pub fn right(&self, offset: u16) -> Point {
        self.area().point.right(offset)
    }

    #[allow(dead_code)]
    pub fn down(&self, offset: u16) -> Point {
        self.area().point.down(offset)
    }

    pub fn draw<T>(&self, text: T, label: Label) -> &Self
    where
        T: AsRef<str> + Clone + Display,
    {
        self.area().point.draw(text, label);
        self
    }
}
