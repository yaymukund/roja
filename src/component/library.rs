use super::{List, ListItem};
use crate::library::{Folder, Library};
use crate::ui::{Event, Listener, State};

impl Listener for Library {
    fn on_event(&self, event: &Event, state: &mut State) {
        let renderer = List::new(point!(0, 0), state.cols() / 3, state.rows() - 1, 0, 0);

        match *event {
            Event::Draw => renderer.draw_rows(self.folders()),
            _ => {}
        }
    }
}

impl ListItem for Folder {
    fn item_text(&self) -> &str {
        self.path_str()
    }
}

// impl FolderList {
//     pub fn new(canvas: Canvas, runtime: RcRuntime) -> Self {
//         FolderList {
//             canvas,
//             selected_index: 0,
//             runtime,
//         }
//     }
//
//     fn draw(&self) {
//         if self.disabled() {
//             return;
//         }
//
//         let folder_count: u16 = self
//             .library()
//             .folders
//             .len()
//             .try_into()
//             .expect("could not turn folders.len() into a u16");
//         let row_count = min(self.canvas.height(), folder_count);
//         for i in 0..row_count {
//             let y = i + self.canvas.y1;
//             let folder = &self.library().folders[i as usize];
//             self.draw_row(folder, y, i == self.selected_index);
//         }
//     }
//
//     fn draw_row(&self, folder: &Folder, num: u16, selected: bool) {
//         let width = self.canvas.width() as usize - 2;
//         let folder_path = &folder.path_str();
//         let width = min(width, folder_path.len());
//         let folder_path = &folder_path[..width];
//
//         if selected {
//             let folder_path = style::style(folder_path.clone())
//                 .with(style::Color::White)
//                 .on(style::Color::DarkMagenta);
//             helpers::write_styled_at(self.canvas.x1 + 1, self.canvas.y1 + num, folder_path);
//         } else {
//             helpers::write_at(self.canvas.x1 + 1, self.canvas.y1 + num, folder_path);
//         }
//     }
//
//     fn library(&self) -> &Library {
//         &self.runtime.borrow().library
//     }
//
//     fn selected_folder(&self) -> &Folder {
//         &self.library().folders[self.selected_index as usize]
//     }
//
//     fn disabled(&self) -> bool {
//         self.canvas.width() < 4
//     }
// }
