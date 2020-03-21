use mpv::events::simple::Event as MpvEvent;
use mpv::{Format, GetData, Mpv, Result, SetData};

pub trait MpvApi {
    fn command(&self, name: &str, args: &[&str]) -> Result<()>;
    fn disable_deprecated_events(&self) -> Result<()>;
    fn get_property<T: GetData>(&self, name: &str) -> Result<T>;
    fn observe_property(&self, name: &str, format: Format, id: u64) -> Result<()>;
    fn set_property(&self, name: &str, value: impl SetData) -> Result<()>;
    fn wait_event(&self, timeout: f64) -> Option<Result<MpvEvent<'_>>>;
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

    fn wait_event(&self, timeout: f64) -> Option<Result<MpvEvent<'_>>> {
        unsafe { Mpv::wait_event(self, timeout) }
    }
}

#[cfg(test)]
pub use self::test::*;

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::{Ref, RefCell};

    pub const MOCK_MP3: &str = "mocks/1-second.mp3";

    #[derive(Debug, PartialEq)]
    pub enum MpvCommand {
        Command(String),
        DisableDeprecatedEvents,
        GetProperty(String),
        ObserveProperty(String),
        SetProperty(String),
        WaitEvent,
    }

    use MpvCommand::*;

    pub struct MockMpv {
        invocations: RefCell<Vec<MpvCommand>>,
        mpv: Mpv,
    }

    impl MockMpv {
        pub fn new() -> MockMpv {
            let mpv = Mpv::new().unwrap();
            let invocations = RefCell::new(Vec::new());

            MockMpv { invocations, mpv }
        }

        fn push_invocation(&self, command: MpvCommand) {
            let mut invocations = self.invocations.borrow_mut();
            invocations.push(command);
        }

        pub fn invocations(&self) -> Ref<'_, Vec<MpvCommand>> {
            self.invocations.borrow()
        }
    }

    impl MpvApi for MockMpv {
        fn command(&self, name: &str, args: &[&str]) -> Result<()> {
            self.push_invocation(Command(name.to_string()));
            Mpv::command(&self.mpv, name, args)
        }

        fn disable_deprecated_events(&self) -> Result<()> {
            self.push_invocation(DisableDeprecatedEvents);
            Mpv::disable_deprecated_events(&self.mpv)
        }

        fn get_property<T: GetData>(&self, name: &str) -> Result<T> {
            self.push_invocation(GetProperty(name.to_string()));
            Mpv::get_property(&self.mpv, name)
        }

        fn observe_property(&self, name: &str, format: Format, id: u64) -> Result<()> {
            self.push_invocation(ObserveProperty(name.to_string()));
            Mpv::observe_property(&self.mpv, name, format, id)
        }

        fn set_property(&self, name: &str, value: impl SetData) -> Result<()> {
            self.push_invocation(SetProperty(name.to_string()));
            Mpv::set_property(&self.mpv, name, value)
        }

        fn wait_event(&self, timeout: f64) -> Option<Result<MpvEvent<'_>>> {
            self.push_invocation(WaitEvent);
            unsafe { Mpv::wait_event(&self.mpv, timeout) }
        }
    }
}
