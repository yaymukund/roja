use super::{Event, Listener, State};

pub trait IntoComponent {
    type IntoComp: Component;
    fn into_component(self, cols: u16, rows: u16) -> Self::IntoComp;
}

pub trait Component {
    /// draw the component
    fn draw(&self) {}

    /// adjust the dimensions. do not redraw
    fn resize(&mut self, _cols: u16, _rows: u16) {}

    /// stuff you want to do every run loop, incl. polling for new events
    fn on_tick(&self, _ui: &mut State) {}

    /// respond to changes
    fn on_event(&mut self, _event: &Event, _ui: &mut State) {}
}

impl<C: Component> Listener for C {
    fn on_event(&mut self, event: &Event, ui: &mut State) {
        match event {
            &Event::Draw => self.draw(),
            &Event::Resize(cols, rows) => self.resize(cols, rows),
            &Event::Tick => self.on_tick(ui),
            event => self::Component::on_event(self, event, ui),
        }
    }
}

impl<C> IntoComponent for C
where
    C: Component,
{
    type IntoComp = C;
    fn into_component(self, _cols: u16, _rows: u16) -> Self::IntoComp {
        self
    }
}
