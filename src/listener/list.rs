use std::ops::RangeInclusive;

use crate::ui::{Event, Label};
use crate::util::{truncate, usize_to_u16, Canvas};

pub trait ListRow {
    fn row_text(&self) -> &str;
}

pub struct List {
    canvas: Canvas,
    start_index: u16,
    selected_index: u16,
    make_canvas: Box<dyn Fn(u16, u16) -> Canvas>,
    disabled: bool,
}

pub struct ListExecutor<'a, R: ListRow> {
    list: &'a mut List,
    items: &'a Vec<R>,
    on_highlight: Option<Box<dyn Fn(&R)>>,
}

impl List {
    pub fn new<F: 'static>(make_canvas: F) -> Self
    where
        F: Fn(u16, u16) -> Canvas,
    {
        Self {
            canvas: Canvas::Uninitialized,
            disabled: false,
            make_canvas: Box::new(make_canvas),
            start_index: 0,
            selected_index: 0,
        }
    }

    fn is_disabled(&self) -> bool {
        self.disabled
    }

    pub fn disable(&mut self) {
        self.disabled = true;
    }

    pub fn enable(&mut self) {
        self.disabled = false;
    }

    pub fn reset(&mut self) {
        self.start_index = 0;
        self.selected_index = 0;
    }

    pub fn items<'a, R>(&'a mut self, items: &'a Vec<R>) -> ListExecutor<'a, R>
    where
        R: ListRow,
    {
        ListExecutor {
            list: self,
            items,
            on_highlight: None,
        }
    }
}

impl<'a, R: ListRow> ListExecutor<'a, R> {
    pub fn on_highlight<F: 'static>(&mut self, on_highlight: F) -> &mut Self
    where
        F: Fn(&R),
    {
        self.on_highlight = Some(Box::new(on_highlight));
        self
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            Event::Draw if self.should_draw() => self.draw_all(),
            Event::Resize(width, height) => self.resize_canvas(*width, *height),
            _ => {}
        }

        if self.list.is_disabled() {
            return;
        }

        let old_selected_index = self.list.selected_index;

        match event {
            Event::MoveDown => self.scroll_down(),
            Event::MoveUp => self.scroll_up(),
            Event::PageDown => self.scroll_page_down(),
            Event::PageUp => self.scroll_page_up(),
            _ => {}
        }

        if let Some(on_highlight) = &self.on_highlight {
            if old_selected_index != self.list.selected_index {
                let item = self.get_item(self.list.selected_index);
                on_highlight(item);
            }
        }
    }

    fn get_item(&self, index: u16) -> &R {
        &self.items[usize::from(index)]
    }

    fn items_len(&self) -> u16 {
        usize_to_u16(self.items.len())
    }

    fn end_index(&self) -> u16 {
        self.list.start_index + self.list.canvas.height() - 1
    }

    fn selected_position(&self) -> u16 {
        self.list.selected_index - self.list.start_index
    }

    fn visible_indices(&self) -> RangeInclusive<u16> {
        self.list.start_index..=self.end_index()
    }

    fn draw_row(&self, position: u16) {
        let index = self.list.start_index + position;
        let total_width = self.list.canvas.width().saturating_sub(2);
        let point = self.list.canvas.point().down(position);

        if index >= self.items_len() {
            let text = &format!(" {:space$} ", "", space = total_width.into());
            point.draw(text, Label::ListRow);
            return;
        }

        let item = self.get_item(index);
        let (text, text_width) = truncate(item.row_text(), total_width);
        let text = &format!(
            " {}{:space$} ",
            text,
            "",
            space = usize::from(total_width - text_width)
        );

        let label = if index == self.list.selected_index {
            Label::ListHighlightedRow
        } else {
            Label::ListRow
        };

        point.draw(text, label);
    }

    fn draw_all(&self) {
        for index in self.visible_indices() {
            self.draw_row(index - self.list.start_index);
        }
    }

    fn scroll_down(&mut self) {
        if self.list.selected_index == self.items_len().saturating_sub(1) {
            return;
        }

        self.select(self.list.selected_index + 1);
    }

    fn scroll_up(&mut self) {
        if self.list.selected_index == 0 {
            return;
        }

        self.select(self.list.selected_index - 1);
    }

    fn scroll_page_down(&mut self) {
        let page_size = self.page_size();
        let mut new_index = self.list.selected_index + page_size;

        if new_index > self.items_len().saturating_sub(1) {
            new_index = self.items_len().saturating_sub(1);
        }

        self.select(new_index);
    }

    fn scroll_page_up(&mut self) {
        let new_index = self.list.selected_index.saturating_sub(self.page_size());
        self.select(new_index);
    }

    fn page_size(&self) -> u16 {
        self.list.canvas.height() / 2
    }

    fn should_draw(&self) -> bool {
        self.list.canvas.is_initialized() && self.list.canvas.width() > 4
    }

    fn select(&mut self, new_index: u16) {
        if self.list.selected_index == new_index {
            return;
        }

        if self.visible_indices().contains(&new_index) {
            let position = self.selected_position();
            self.list.selected_index = new_index;
            self.draw_row(position);
            self.draw_row(self.selected_position());
        } else {
            if new_index > self.list.selected_index {
                self.list.start_index += new_index - self.list.selected_index;
                self.list.selected_index = new_index;
            } else {
                self.list.start_index = self
                    .list
                    .start_index
                    .saturating_sub(self.list.selected_index - new_index);
                self.list.selected_index = new_index;
            }

            let max_start_index = self.items_len().saturating_sub(self.list.canvas.height());
            if self.list.start_index > max_start_index {
                self.list.start_index = max_start_index;
            }

            self.draw_all();
        }
    }

    fn resize_canvas(&mut self, width: u16, height: u16) {
        self.list.canvas = (&self.list.make_canvas)(width, height);
    }
}
