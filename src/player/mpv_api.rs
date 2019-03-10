use mpv::events::simple::Event;
use mpv::{Format, GetData, Mpv, Result, SetData};

pub trait MpvApi {
    fn command(&self, name: &str, args: &[&str]) -> Result<()>;
    fn disable_deprecated_events(&self) -> Result<()>;
    fn get_property<T: GetData>(&self, name: &str) -> Result<T>;
    fn observe_property(&self, name: &str, format: Format, id: u64) -> Result<()>;
    fn set_property(&self, name: &str, value: impl SetData) -> Result<()>;
    fn wait_event(&self, timeout: f64) -> Option<Result<Event>>;
}

impl MpvApi for Mpv {
    fn command(&self, name: &str, args: &[&str]) -> Result<()> {
        Mpv::command(self, name, args)
    }

    fn disable_deprecated_events(&self) -> Result<()> {
        Mpv::disable_deprecated_events(self)
    }

    fn get_property<T: GetData>(&self, name: &str) -> Result<T> {
        Mpv::get_property(self, name)
    }

    fn observe_property(&self, name: &str, format: Format, id: u64) -> Result<()> {
        Mpv::observe_property(self, name, format, id)
    }

    fn set_property(&self, name: &str, value: impl SetData) -> Result<()> {
        Mpv::set_property(self, name, value)
    }

    fn wait_event(&self, timeout: f64) -> Option<Result<Event>> {
        unsafe { Mpv::wait_event(self, timeout) }
    }
}
