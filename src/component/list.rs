use std::cmp::min;
use std::ops::RangeInclusive;

use crossterm::{style, style::Styler};

use crate::ui::{Component, Event, State};
use crate::util::{truncate, usize_to_u16, Canvas};

pub trait ListRow {
    fn row_text(&self) -> &str;
}

pub trait Listable {
    type RowItem: ListRow;
    fn items(&self) -> &[Self::RowItem];
    fn on_select(&mut self);
    fn canvas(cols: u16, rows: u16) -> Canvas;
}

pub struct List<L: Listable> {
    listable: L,
    canvas: Canvas,
    start_index: u16,
    selected_index: u16,
}

impl<L> List<L>
where
    L: Listable,
{
    pub fn new(listable: L, canvas: Canvas) -> Self {
        Self {
            listable,
            canvas,
            start_index: 0,
            selected_index: 0,
        }
    }

    fn items_count(&self) -> u16 {
        usize_to_u16(self.listable.items().len())
    }

    fn get_item(&self, index: u16) -> &L::RowItem {
        &self.listable.items()[usize::from(index)]
    }

    fn end_index(&self) -> u16 {
        let end = self.start_index + self.canvas.height() - 1;
        min(end, self.items_count() - 1)
    }

    fn selected_position(&self) -> u16 {
        self.selected_index - self.start_index
    }

    fn visible_indices(&self) -> RangeInclusive<u16> {
        self.start_index..=self.end_index()
    }

    fn draw_row(&self, position: u16) {
        let index = self.start_index + position;
        let item = self.get_item(index);
        let total_width = self.canvas.width().saturating_sub(2);
        let (text, text_width) = truncate(item.row_text(), total_width);
        let text = &format!(
            " {}{:rem$} ",
            text,
            "",
            rem = usize::from(total_width - text_width)
        );
        let point = self.canvas.point().down(position);

        if index == self.selected_index {
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
        for index in self.visible_indices() {
            self.draw_row(index - self.start_index);
        }
    }

    fn scroll_down(&mut self) {
        if self.selected_index == self.items_count() - 1 {
            return;
        }

        self.select(self.selected_index + 1);
    }

    fn scroll_up(&mut self) {
        if self.selected_index == 0 {
            return;
        }

        self.select(self.selected_index - 1);
    }

    fn scroll_page_down(&mut self) {
        let page_size = self.page_size();
        let items_count = self.items_count();
        let mut new_index = self.selected_index + page_size;

        if new_index > items_count - 1 {
            new_index = items_count - 1;
        }

        self.select(new_index);
    }

    fn scroll_page_up(&mut self) {
        let new_index = self.selected_index.saturating_sub(self.page_size());
        self.select(new_index);
    }

    fn page_size(&self) -> u16 {
        self.canvas.height() / 2
    }

    fn should_render(&self) -> bool {
        self.canvas.width() > 4
    }

    fn select(&mut self, new_index: u16) {
        if self.selected_index == new_index {
            return;
        }

        if self.visible_indices().contains(&new_index) {
            let position = self.selected_position();
            self.selected_index = new_index;
            self.draw_row(position);
            self.draw_row(self.selected_position());
            return;
        }

        if new_index > self.selected_index {
            self.start_index += new_index - self.selected_index;
            self.selected_index = new_index;
        } else {
            self.start_index = self
                .start_index
                .saturating_sub(self.selected_index - new_index);
            self.selected_index = new_index;
        }

        let max_start_index = self.items_count() - self.canvas.height();
        if self.start_index > max_start_index {
            self.start_index = max_start_index;
        }

        self.draw_all();
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
            Event::MoveDown => self.scroll_down(),
            Event::MoveUp => self.scroll_up(),
            Event::PageDown => self.scroll_page_down(),
            Event::PageUp => self.scroll_page_up(),
            _ => {}
        }
    }
}
