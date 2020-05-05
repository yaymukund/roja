use std::cmp::min;

use crossterm::{style, style::Styler};

use crate::ui::{Event, Listener, State};
use crate::util::{usize_to_u16, Point};

pub trait ListRow {
    fn row_text(&self) -> &str;
}

pub trait ListStore {
    type RowItem: ListRow;
    fn get(&self, index: usize) -> &Self::RowItem;
    fn count(&self) -> usize;
    fn on_select(&mut self);
}

pub struct List<S> {
    store: S,
    point: Point,
    width: u16,
    height: u16,
    start_index: usize,
    selected_index: usize,
}

impl<S> List<S>
where
    S: ListStore,
{
    pub fn new(store: S, point: Point, width: u16, height: u16) -> Self {
        Self {
            store,
            point,
            width,
            height,
            start_index: 0,
            selected_index: 0,
        }
    }

    fn draw_row(&self, index: usize, selected: bool) {
        let item = self.store.get(index);
        let text = item.row_text();
        let text_width = min(self.width.into(), text.len()).saturating_sub(2);
        let text = &text[..text_width];
        let width = usize::from(self.width - 1);
        let text = &format!(" {:width$}", text, width = width);
        let point = self.point.down(usize_to_u16(index));

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

    fn draw(&self) {
        let remaining_items_count = self.store.count() - self.start_index;
        let row_count = min(self.height.into(), remaining_items_count);
        for i in 0..row_count {
            self.draw_row(i, self.selected_index == i);
        }
    }

    fn move_down(&mut self) {
        if self.selected_index == self.store.count() {
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
        self.width > 4
    }
}

impl<S> Listener for List<S>
where
    S: ListStore,
{
    fn on_event(&mut self, event: &Event, _state: &mut State) {
        if !self.should_render() {
            return;
        }

        match *event {
            Event::Draw => self.draw(),
            Event::MoveDown => self.move_down(),
            Event::MoveUp => self.move_up(),
            _ => {}
        }
    }
}
