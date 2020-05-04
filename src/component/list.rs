use std::cmp::min;

use crate::util::Point;

pub struct List {
    point: Point,
    width: u16,
    height: u16,
    start_index: u16,
    selected_index: u16,
}

pub trait ListItem {
    fn item_text(&self) -> &str;
}

impl List {
    pub fn new(
        point: Point,
        width: u16,
        height: u16,
        start_index: u16,
        selected_index: u16,
    ) -> Self {
        Self {
            point,
            width,
            height,
            start_index,
            selected_index,
        }
    }

    fn draw_row(&self, num: u16, item: &impl ListItem, _selected: bool) {
        let text = item.item_text();
        let width = min(self.width.into(), text.len());
        let text = &text[..width];

        self.point.down(num).write(text);
    }

    pub fn draw_rows(&self, items: &[impl ListItem]) {
        let remaining_items_count = items.len() as u16 - self.start_index;
        let row_count = min(self.height, remaining_items_count);
        for i in 0..row_count as u16 {
            let item = &items[(self.start_index + i) as usize];
            self.draw_row(i, item, false);
        }
    }
}
