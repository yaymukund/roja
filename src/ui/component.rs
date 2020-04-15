use crate::runtime::RcRuntime;
use crate::ui::UIEvent;

pub(crate) struct Canvas {
    pub(crate) x1: u16,
    pub(crate) y1: u16,
    pub(crate) x2: u16,
    pub(crate) y2: u16,
}

impl Canvas {
    pub(crate) fn width(&self) -> u16 {
        self.x2 - self.x1
    }

    pub(crate) fn height(&self) -> u16 {
        self.y2 - self.y1
    }
}

pub(crate) trait UIComponent {
    fn on_event(&self, event: &UIEvent, runtime: RcRuntime);
    fn after_event(&self, event: &UIEvent, runtime: RcRuntime);
}
