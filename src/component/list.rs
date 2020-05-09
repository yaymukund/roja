use std::cmp::min;
use std::ops::Range;

use crossterm::{style, style::Styler};

use crate::ui::{Component, Event, State};
use crate::util::{truncate, usize_to_u16, Canvas};

const LIST_BUFFER: u16 = 3;

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

    fn item_position(&self, index: usize) -> u16 {
        usize_to_u16(index - self.start_index)
    }

    fn visible_range(&self) -> Range<usize> {
        let remaining = self.items.count() - self.start_index;
        let height: usize = self.canvas.height().into();
        let end_index = min(remaining, height + self.start_index);
        self.start_index..end_index
    }

    fn draw_row(&self, index: usize, selected: bool) {
        let item = self.items.get(index);
        let position = self.item_position(index);
        let total_width: usize = self.canvas.width().saturating_sub(2).into();
        let (text, text_width) = truncate(item.row_text(), total_width);
        let text = &format!(" {}{:rem$} ", text, "", rem = (total_width - text_width));
        let point = self.canvas.point().down(position);

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
        for index in self.visible_range() {
            self.draw_row(index, self.selected_index == index);
        }
    }

    fn move_down(&mut self) {
        let index = self.selected_index;
        if self.selected_index == self.items.count() - 1 {
            return;
        }

        let new_index = index + 1;
        self.selected_index += 1;

        if self.visible_range().contains(&new_index) {
            self.draw_row(index, false);
            self.draw_row(new_index, true);
        } else {
            self.start_index += 1;
            self.draw_all();
        }
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
