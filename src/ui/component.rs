use crate::runtime::RcRuntime;
use crate::ui::UIEvent;

pub(crate) struct Canvas {
    pub(crate) x: u16,
    pub(crate) y: u16,
    pub(crate) rows: u16,
    pub(crate) cols: u16,
}

pub(crate) trait UIComponent {
    fn on_event(&self, event: &UIEvent, runtime: RcRuntime);
    fn after_event(&self, event: &UIEvent, runtime: RcRuntime);
}
