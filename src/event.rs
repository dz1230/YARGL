
pub enum EventReturnCode {
    Continue,
    Cancel,
    Quit
}

pub struct Event<T>(T);

pub trait EventReceiver<T> {
    fn on(&mut self, event: &Event<T>) -> EventReturnCode;
}