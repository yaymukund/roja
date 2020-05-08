use std::cmp::min;

use crossterm::{style, style::Styler};

use crate::ui::{Component, Event, State};
use crate::util::{usize_to_u16, Canvas};

pub trait ListRow {
    fn row_text(&self) -> &str;
}

pub trait Listable {
    type RowItem: ListRow;
    fn get(&self, index: usize) -> &Self::RowItem;
    fn count(&self) -> usize;
    fn on_select(&mut self);
    fn canvas(cols: u16, rows: u16) -> Canvas;
}

pub struct List<L: Listable> {
    items: L,
    canvas: Canvas,
    start_index: usize,
    selected_index: usize,
}

impl<L> List<L>
where
    L: Listable,
{
    pub fn new(items: L, canvas: Canvas) -> Self {
        Self {
            items,
            canvas,
            start_index: 0,
            selected_index: 0,
        }
    }

    fn draw_row(&self, index: usize, selected: bool) {
        let item = self.items.get(index);
        let text = item.row_text();
        let text_width = min(self.canvas.width().saturating_sub(2).into(), text.len());
        let text = &text[..text_width];
        let width = usize::from(self.canvas.width() - 1);
        let text = &format!(" {:width$}", text, width = width);
        let point = self.canvas.point().down(usize_to_u16(index));

        if selected {
            let text = style::style(text)
                .bold()
                .with(style::Color::White)
                .on(style::Color::Magenta);
            point.write_styled(text);
        } else {
            point.write(text);
        }
    }

    fn draw_all(&self) {
        let remaining_items_count = self.items.count() - self.start_index;
        let row_count = min(self.canvas.height().into(), remaining_items_count);
        for i in 0..row_count {
            self.draw_row(i, self.selected_index == i);
        }
    }

    fn move_down(&mut self) {
        if self.selected_index == self.items.count() {
            return;
        }

        self.draw_row(self.selected_index, false);
        self.selected_index += 1;
        self.draw_row(self.selected_index, true);
    }

    fn move_up(&mut self) {
        if self.selected_index == 0 {
            return;
        }

        self.draw_row(self.selected_index, false);
        self.selected_index -= 1;
        self.draw_row(self.selected_index, true);
    }

    fn should_render(&self) -> bool {
        self.canvas.width() > 4
    }
}

impl<L> Component for List<L>
where
    L: Listable,
{
    fn draw(&self) {
        self.draw_all();
    }

    fn resize(&mut self, cols: u16, rows: u16) {
        self.canvas = L::canvas(cols, rows);
    }

    fn on_event(&mut self, event: &Event, _state: &mut State) {
        if !self.should_render() {
            return;
        }

        match *event {
            Event::MoveDown => self.move_down(),
            Event::MoveUp => self.move_up(),
            _ => {}
        }
    }
}
